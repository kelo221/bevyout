//! RCPE preparation and the content-addressed recipe catalogue (M3 wave 7).

use super::super::recipe::{
    PreparedRecipe, PreparedRecipeCatalog, PreparedRecipeItem, validate_recipe,
};
use super::*;
use std::collections::BTreeSet;

pub(crate) const RECIPE_CATALOG_REVISION: &str = "openmw-recipes-v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecipeCatalogIssue {
    pub(crate) form_id: u32,
    pub(crate) message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecipeCatalogBuild {
    pub(crate) catalog: PreparedRecipeCatalog,
    pub(crate) invalid: Vec<RecipeCatalogIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecipeCatalogArtifact {
    pub(crate) relative_path: String,
    pub(crate) hash: String,
    pub(crate) reused: bool,
}

/// Converts parsed RCPE records in FormID order and excludes each invalid
/// record as one unit.  The validation itself remains in the serde/std-only
/// `recipe` module so a future crafting transaction can apply the same policy
/// before mutating inventory.
pub(crate) fn build_recipe_catalog(
    recipes: &HashMap<u32, RecipeRecord>,
    bases: &HashMap<u32, BaseRecord>,
    source_fingerprint: &str,
) -> RecipeCatalogBuild {
    let available_items = bases.keys().copied().collect::<BTreeSet<_>>();
    let mut recipe_ids = recipes.keys().copied().collect::<Vec<_>>();
    recipe_ids.sort_unstable();

    let mut prepared = Vec::new();
    let mut invalid = Vec::new();
    for form_id in recipe_ids {
        let source = &recipes[&form_id];
        let candidate = prepared_recipe(source);
        if let Err(error) = validate_recipe(&candidate, &available_items) {
            invalid.push(RecipeCatalogIssue {
                form_id,
                message: error.to_string(),
            });
        } else {
            prepared.push(candidate);
        }
    }

    RecipeCatalogBuild {
        catalog: PreparedRecipeCatalog {
            revision: RECIPE_CATALOG_REVISION.into(),
            source_fingerprint: source_fingerprint.into(),
            recipes: prepared,
        },
        invalid,
    }
}

fn prepared_recipe(source: &RecipeRecord) -> PreparedRecipe {
    let mut ingredients = source
        .ingredients
        .iter()
        .map(prepared_recipe_item)
        .collect::<Vec<_>>();
    let mut outputs = source
        .outputs
        .iter()
        .map(prepared_recipe_item)
        .collect::<Vec<_>>();
    sort_recipe_items(&mut ingredients);
    sort_recipe_items(&mut outputs);
    PreparedRecipe {
        form_id: source.form_id,
        record_flags: source.record_flags,
        editor_id: source.editor_id.clone(),
        display_name: source.name.clone(),
        skill: source.skill,
        level: source.level,
        category_form_id: source.category_form_id,
        sub_category_form_id: source.sub_category_form_id,
        ingredients,
        outputs,
        conditions: source.conditions.clone(),
    }
}

fn prepared_recipe_item(source: &RecipeItemRecord) -> PreparedRecipeItem {
    PreparedRecipeItem {
        item_form_id: source.item_form_id,
        quantity: source.quantity,
        order: source.order,
    }
}

fn sort_recipe_items(items: &mut [PreparedRecipeItem]) {
    items.sort_by_key(|item| (item.item_form_id, item.order, item.quantity));
}

/// Writes the deterministic recipe artifact below the content fingerprint.
/// A byte-identical existing file is left untouched and reported as reused;
/// this makes repeated cell preparation cheap while retaining the item's
/// established path/hash manifest contract.
pub(crate) fn write_recipe_catalog(
    cache_dir: &Path,
    catalog: &PreparedRecipeCatalog,
) -> Result<RecipeCatalogArtifact> {
    let relative = PathBuf::from("catalogs")
        .join(&catalog.source_fingerprint)
        .join("recipes.ron");
    let path = cache_dir.join(&relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let serialized = to_string_pretty(catalog, PrettyConfig::default())?;
    let hash = fingerprint(serialized.as_bytes());
    let reused = fs::read(&path)
        .map(|existing| existing == serialized.as_bytes())
        .unwrap_or(false);
    if !reused {
        fs::write(&path, serialized)?;
    }
    Ok(RecipeCatalogArtifact {
        relative_path: relative.to_string_lossy().replace('\\', "/"),
        hash,
        reused,
    })
}
