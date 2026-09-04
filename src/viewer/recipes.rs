//! Prepared schematic catalog loaded at viewer startup (M9 wave 5).

use bevy::prelude::*;
use bevyout_core::crafting::{RecipeDefinition, RecipeItem};
use std::collections::BTreeMap;
use std::path::Path;

use crate::vsa::{PreparedRecipeCatalog, PreparedSceneManifest, RECIPE_CATALOG_REVISION};

#[derive(Resource, Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct RecipeCatalog(pub(crate) BTreeMap<u32, RecipeDefinition>);

pub(crate) fn load_recipe_catalog_for_manifest(
    manifest: &PreparedSceneManifest,
    asset_root: &Path,
) -> RecipeCatalog {
    let path = asset_root
        .join("catalogs")
        .join(&manifest.source_fingerprint)
        .join("recipes.ron");
    let Some(text) = std::fs::read_to_string(&path).ok() else {
        warn!("recipes: no recipe catalog for this content set, craftitem disabled");
        return RecipeCatalog::default();
    };
    let catalog: PreparedRecipeCatalog = match ron::from_str(&text) {
        Ok(catalog) => catalog,
        Err(error) => {
            warn!("recipes: recipe catalog unreadable ({error}), craftitem disabled");
            return RecipeCatalog::default();
        }
    };
    if catalog.revision != RECIPE_CATALOG_REVISION {
        warn!(
            "recipes: recipe catalog revision {} is stale, expected {RECIPE_CATALOG_REVISION}; run `prepare` again (craftitem disabled)",
            catalog.revision
        );
        return RecipeCatalog::default();
    }
    if catalog.source_fingerprint != manifest.source_fingerprint {
        warn!("recipes: recipe catalog fingerprint mismatch, craftitem disabled");
        return RecipeCatalog::default();
    }
    info!(
        "recipes: loaded {} schematics from recipe catalog",
        catalog.recipes.len()
    );
    RecipeCatalog(
        catalog
            .recipes
            .into_iter()
            .map(|recipe| {
                (
                    recipe.form_id,
                    RecipeDefinition {
                        form_id: recipe.form_id,
                        skill: recipe.skill,
                        level: recipe.level,
                        ingredients: recipe
                            .ingredients
                            .into_iter()
                            .filter(|item| item.quantity > 0)
                            .map(|item| RecipeItem {
                                item_form_id: item.item_form_id,
                                quantity: item.quantity as u32,
                                order: item.order,
                            })
                            .collect(),
                        outputs: recipe
                            .outputs
                            .into_iter()
                            .filter(|item| item.quantity > 0)
                            .map(|item| RecipeItem {
                                item_form_id: item.item_form_id,
                                quantity: item.quantity as u32,
                                order: item.order,
                            })
                            .collect(),
                        has_conditions: !recipe.conditions.is_empty(),
                    },
                )
            })
            .collect(),
    )
}
