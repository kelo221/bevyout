//! GRUP traversal and record dispatch.

use super::*;

/// Which cell/worldspace the current position in the byte stream is nested
/// under, threaded through recursive `walk_container` calls. Bundled into
/// one struct (rather than two more parameters) to keep `walk_container`
/// under clippy's argument-count lint.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct GroupContext {
    /// FormID of the enclosing GRUP type-6 ("Cell Children") group, if any.
    pub(crate) cell: Option<u32>,
    /// FormID of the enclosing GRUP type-1 ("World Children") group's WRLD,
    /// if any.
    pub(crate) worldspace: Option<u32>,
}

pub(crate) fn walk_container(
    bytes: &[u8],
    mut offset: usize,
    end: usize,
    context: GroupContext,
    resolver: &FormIdResolver,
    state: &mut ParsedState,
    source_name: &str,
) -> Result<()> {
    while offset + 4 <= end {
        let signature = &bytes[offset..offset + 4];
        if signature == b"GRUP" {
            if offset + 24 > end {
                bail!("truncated GRUP header")
            }
            let size = read_u32(bytes, offset + 4)? as usize;
            if size < 24 || offset + size > end {
                bail!("invalid GRUP size")
            }
            let label = read_u32(bytes, offset + 8)?;
            let group_type = read_i32(bytes, offset + 12)?;
            let child_cell = if group_type == 6 {
                Some(resolver.adjust(label))
            } else {
                context.cell
            };
            // GRUP type 1 ("World Children") labels its FormID with the
            // owning WRLD record; every CELL found underneath (through the
            // nested type 4/5 exterior block/sub-block groups) belongs to
            // that worldspace. Types 0 (top), 2/3 (interior block/sub-block)
            // and 6 (cell children) leave the current worldspace unchanged.
            let child_worldspace = if group_type == 1 {
                Some(resolver.adjust(label))
            } else {
                context.worldspace
            };
            walk_container(
                bytes,
                offset + 24,
                offset + size,
                GroupContext {
                    cell: child_cell,
                    worldspace: child_worldspace,
                },
                resolver,
                state,
                source_name,
            )?;
            offset += size;
            continue;
        }

        if offset + 24 > end {
            bail!("truncated record header")
        }
        let data_size = read_u32(bytes, offset + 4)? as usize;
        let flags = read_u32(bytes, offset + 8)?;
        let raw_form_id = read_u32(bytes, offset + 12)?;
        let form_id = resolver.adjust(raw_form_id);
        let record_end = offset + 24 + data_size;
        if record_end > end {
            bail!("record exceeds containing group")
        }
        let sig = String::from_utf8_lossy(signature).to_string();
        let data = record_payload(&bytes[offset + 24..record_end], flags, &sig, form_id)?;
        let subs = parse_subrecords(&data)?;

        match sig.as_str() {
            "CELL" => {
                if flags & RECORD_DELETED != 0 {
                    state.cells.remove(&form_id);
                    state.cell_metadata.remove(&form_id);
                    state.cell_winning_plugins.remove(&form_id);
                    state.cell_provenance.remove(&form_id);
                } else {
                    let mut cell = parse_cell(&subs, form_id, resolver)?;
                    cell.worldspace_form_id = context.worldspace;
                    state.cells.insert(form_id, cell);
                    state
                        .cell_metadata
                        .insert(form_id, parse_cell_metadata(&subs, resolver));
                    state
                        .cell_winning_plugins
                        .insert(form_id, source_name.to_string());
                    state
                        .cell_provenance
                        .entry(form_id)
                        .or_default()
                        .push(source_name.to_string());
                }
            }
            "WRLD" => {
                if flags & RECORD_DELETED != 0 {
                    state.worldspaces.remove(&form_id);
                } else {
                    state
                        .worldspaces
                        .insert(form_id, parse_worldspace(&subs, form_id));
                }
            }
            "IMGS" => {
                if flags & RECORD_DELETED != 0 {
                    state.image_spaces.remove(&form_id);
                } else if let Some(image_space) = parse_image_space(&subs, form_id) {
                    state.image_spaces.insert(form_id, image_space);
                }
            }
            "SOUN" => {
                if flags & RECORD_DELETED != 0 {
                    state.sounds.remove(&form_id);
                } else {
                    state
                        .sounds
                        .insert(form_id, parse_sound(&subs, form_id, flags));
                }
            }
            "SNDR" => {
                if flags & RECORD_DELETED != 0 {
                    state.sound_references.remove(&form_id);
                } else {
                    state.sound_references.insert(
                        form_id,
                        parse_sound_reference(&subs, form_id, flags, resolver),
                    );
                }
            }
            "ASPC" => {
                if flags & RECORD_DELETED != 0 {
                    state.acoustic_spaces.remove(&form_id);
                } else {
                    state.acoustic_spaces.insert(
                        form_id,
                        parse_acoustic_space(&subs, form_id, flags, resolver),
                    );
                }
            }
            "MUSC" => {
                if flags & RECORD_DELETED != 0 {
                    state.music.remove(&form_id);
                } else {
                    state
                        .music
                        .insert(form_id, parse_music(&subs, form_id, flags));
                }
            }
            "LGTM" => {
                if flags & RECORD_DELETED != 0 {
                    state.lighting_templates.remove(&form_id);
                } else {
                    state
                        .lighting_templates
                        .insert(form_id, parse_lighting_template(&subs, form_id, flags));
                }
            }
            "RCPE" => {
                if flags & RECORD_DELETED != 0 {
                    state.recipes.remove(&form_id);
                } else {
                    match parse_recipe(&subs, form_id, flags, resolver) {
                        Ok(recipe) => {
                            for signature in &recipe.ignored_subrecords {
                                state.recipe_diagnostics.push(format!(
                                    "{source_name} RCPE {form_id:08x}: ignored unsupported {signature} subrecord"
                                ));
                            }
                            state.recipes.insert(form_id, recipe);
                        }
                        Err(error) => {
                            // A malformed override must not leave an older
                            // recipe active under the same FormID. Keep only
                            // a stable diagnostic and no partial recipe state.
                            state.recipes.remove(&form_id);
                            state.recipe_diagnostics.push(format!(
                                "{source_name} RCPE {form_id:08x}: malformed recipe: {error}"
                            ));
                        }
                    }
                }
            }
            "RACE" => {
                if flags & RECORD_DELETED != 0 {
                    state.races.remove(&form_id);
                } else {
                    match parse_race(&subs, form_id, flags, resolver) {
                        Ok(race) => {
                            for signature in &race.ignored_subrecords {
                                state.actor_support_diagnostics.push(format!(
                                    "{source_name} RACE {form_id:08x}: ignored unsupported {signature} subrecord"
                                ));
                            }
                            state.races.insert(form_id, race);
                        }
                        Err(error) => {
                            state.races.remove(&form_id);
                            state.actor_support_diagnostics.push(format!(
                                "{source_name} RACE {form_id:08x}: malformed race: {error}"
                            ));
                        }
                    }
                }
            }
            "CLAS" => {
                if flags & RECORD_DELETED != 0 {
                    state.classes.remove(&form_id);
                } else {
                    match parse_class(&subs, form_id, flags) {
                        Ok(class) => {
                            for signature in &class.ignored_subrecords {
                                state.actor_support_diagnostics.push(format!(
                                    "{source_name} CLAS {form_id:08x}: ignored unsupported {signature} subrecord"
                                ));
                            }
                            state.classes.insert(form_id, class);
                        }
                        Err(error) => {
                            state.classes.remove(&form_id);
                            state.actor_support_diagnostics.push(format!(
                                "{source_name} CLAS {form_id:08x}: malformed class: {error}"
                            ));
                        }
                    }
                }
            }
            "FACT" => {
                if flags & RECORD_DELETED != 0 {
                    state.factions.remove(&form_id);
                } else {
                    match parse_faction(&subs, form_id, flags, resolver) {
                        Ok(faction) => {
                            for signature in &faction.ignored_subrecords {
                                state.actor_support_diagnostics.push(format!(
                                    "{source_name} FACT {form_id:08x}: ignored unsupported {signature} subrecord"
                                ));
                            }
                            state.factions.insert(form_id, faction);
                        }
                        Err(error) => {
                            state.factions.remove(&form_id);
                            state.actor_support_diagnostics.push(format!(
                                "{source_name} FACT {form_id:08x}: malformed faction: {error}"
                            ));
                        }
                    }
                }
            }
            "PACK" => {
                if flags & RECORD_DELETED != 0 {
                    state.packages.remove(&form_id);
                } else {
                    match parse_package(&subs, form_id, flags, resolver) {
                        Ok(package) => {
                            for signature in &package.ignored_subrecords {
                                state.actor_support_diagnostics.push(format!(
                                    "{source_name} PACK {form_id:08x}: ignored unsupported {signature} subrecord"
                                ));
                            }
                            state.packages.insert(form_id, package);
                        }
                        Err(error) => {
                            state.packages.remove(&form_id);
                            state.actor_support_diagnostics.push(format!(
                                "{source_name} PACK {form_id:08x}: malformed package: {error}"
                            ));
                        }
                    }
                }
            }
            "REFR" | "ACHR" | "ACRE" if context.cell.is_some() => {
                if flags & RECORD_DELETED != 0 {
                    state.references.remove(&form_id);
                } else if let Some(reference) = parse_reference(
                    &subs,
                    form_id,
                    context.cell.unwrap_or_default(),
                    flags,
                    match sig.as_str() {
                        "ACHR" => ReferenceKind::Npc,
                        "ACRE" => ReferenceKind::Creature,
                        _ => ReferenceKind::Object,
                    },
                    resolver,
                )? {
                    state.references.insert(form_id, reference);
                }
            }
            "NAVM" if context.cell.is_some() => {
                if flags & RECORD_DELETED != 0 {
                    state.navmeshes.remove(&form_id);
                } else {
                    state.navmeshes.insert(
                        form_id,
                        (
                            context.cell.unwrap_or_default(),
                            parse_navmesh(&subs, form_id, flags, resolver, data),
                        ),
                    );
                }
            }
            "NAVI" => {
                if flags & RECORD_DELETED != 0 {
                    state.navigation = None;
                } else {
                    let (navigation, diagnostics) = parse_navi(&subs, form_id, flags, resolver);
                    for message in diagnostics {
                        state
                            .navigation_diagnostics
                            .push(format!("{source_name} NAVI {form_id:08x}: {message}"));
                    }
                    state.navigation = Some(navigation);
                }
            }
            _ => {
                if flags & RECORD_DELETED != 0 {
                    state.bases.remove(&form_id);
                } else if let Some(mut base) = parse_base(&sig, &subs, resolver) {
                    base.record_flags = flags;
                    state.bases.insert(form_id, base);
                }
            }
        }
        offset = record_end;
    }
    Ok(())
}
