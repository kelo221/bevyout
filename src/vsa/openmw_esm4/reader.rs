//! GRUP traversal and record dispatch.

use super::*;

pub(crate) fn walk_container(
    bytes: &[u8],
    mut offset: usize,
    end: usize,
    current_cell: Option<u32>,
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
                current_cell
            };
            walk_container(
                bytes,
                offset + 24,
                offset + size,
                child_cell,
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
                    state
                        .cells
                        .insert(form_id, parse_cell(&subs, form_id, resolver)?);
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
            "REFR" | "ACHR" | "ACRE" if current_cell.is_some() => {
                if flags & RECORD_DELETED != 0 {
                    state.references.remove(&form_id);
                } else if let Some(reference) = parse_reference(
                    &subs,
                    form_id,
                    current_cell.unwrap_or_default(),
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
            "NAVM" if current_cell.is_some() => {
                if flags & RECORD_DELETED != 0 {
                    state.navmeshes.remove(&form_id);
                } else {
                    state.navmeshes.insert(
                        form_id,
                        (
                            current_cell.unwrap_or_default(),
                            parse_navmesh(&subs, form_id, flags, data),
                        ),
                    );
                }
            }
            _ => {
                if flags & RECORD_DELETED != 0 {
                    state.bases.remove(&form_id);
                } else if let Some(base) = parse_base(&sig, &subs, resolver) {
                    state.bases.insert(form_id, base);
                }
            }
        }
        offset = record_end;
    }
    Ok(())
}
