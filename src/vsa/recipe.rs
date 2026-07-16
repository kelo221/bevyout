//! Prepared recipe data and the pure validation policy used by preparation.
//!
//! This module deliberately has no Bevy or filesystem dependency.  The ESM4
//! reader and the prepare slice adapt their records into these serde-owned
//! values, while future crafting transactions can reuse the validation
//! policy without importing either runtime.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedRecipeCatalog {
    #[serde(default)]
    pub(crate) revision: String,
    #[serde(default)]
    pub(crate) source_fingerprint: String,
    #[serde(default)]
    pub(crate) recipes: Vec<PreparedRecipe>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct PreparedRecipe {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) display_name: Option<String>,
    pub(crate) skill: i32,
    pub(crate) level: u32,
    pub(crate) category_form_id: Option<u32>,
    pub(crate) sub_category_form_id: Option<u32>,
    pub(crate) ingredients: Vec<PreparedRecipeItem>,
    pub(crate) outputs: Vec<PreparedRecipeItem>,
    /// Opaque CTDA payloads are retained for a later condition evaluator.
    /// Keeping them here prevents preparation from inventing or dropping
    /// recipe restrictions while crafting execution remains out of scope.
    pub(crate) conditions: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct PreparedRecipeItem {
    pub(crate) item_form_id: u32,
    pub(crate) quantity: i32,
    /// Original pair order, retained as a deterministic tie-break key.
    pub(crate) order: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RecipeValidationSide {
    Ingredient,
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RecipeValidationError {
    MissingIngredients,
    MissingOutputs,
    DuplicateIngredient {
        item_form_id: u32,
    },
    NonPositiveQuantity {
        side: RecipeValidationSide,
        item_form_id: u32,
        quantity: i32,
    },
    MissingIngredientItem {
        item_form_id: u32,
    },
    MissingOutputItem {
        item_form_id: u32,
    },
}

impl fmt::Display for RecipeValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingIngredients => formatter.write_str("recipe has no ingredients"),
            Self::MissingOutputs => formatter.write_str("recipe has no outputs"),
            Self::DuplicateIngredient { item_form_id } => {
                write!(formatter, "duplicate ingredient item {item_form_id:08x}")
            }
            Self::NonPositiveQuantity {
                side,
                item_form_id,
                quantity,
            } => write!(
                formatter,
                "{} item {item_form_id:08x} has non-positive quantity {quantity}",
                match side {
                    RecipeValidationSide::Ingredient => "ingredient",
                    RecipeValidationSide::Output => "output",
                }
            ),
            Self::MissingIngredientItem { item_form_id } => {
                write!(formatter, "missing ingredient item {item_form_id:08x}")
            }
            Self::MissingOutputItem { item_form_id } => {
                write!(formatter, "missing output item {item_form_id:08x}")
            }
        }
    }
}

impl std::error::Error for RecipeValidationError {}

/// Validates one prepared recipe without mutating the recipe or the item set.
/// Callers can therefore reject the whole future transaction before consuming
/// any inventory.  The catalog builder uses this same policy and excludes an
/// invalid recipe as one unit.
pub(crate) fn validate_recipe(
    recipe: &PreparedRecipe,
    available_items: &BTreeSet<u32>,
) -> Result<(), RecipeValidationError> {
    if recipe.outputs.is_empty() {
        return Err(RecipeValidationError::MissingOutputs);
    }
    if recipe.ingredients.is_empty() {
        return Err(RecipeValidationError::MissingIngredients);
    }

    let mut ingredient_ids = BTreeSet::new();
    for item in &recipe.ingredients {
        if !ingredient_ids.insert(item.item_form_id) {
            return Err(RecipeValidationError::DuplicateIngredient {
                item_form_id: item.item_form_id,
            });
        }
    }

    for item in &recipe.ingredients {
        if item.quantity <= 0 {
            return Err(RecipeValidationError::NonPositiveQuantity {
                side: RecipeValidationSide::Ingredient,
                item_form_id: item.item_form_id,
                quantity: item.quantity,
            });
        }
        if !available_items.contains(&item.item_form_id) {
            return Err(RecipeValidationError::MissingIngredientItem {
                item_form_id: item.item_form_id,
            });
        }
    }
    for item in &recipe.outputs {
        if item.quantity <= 0 {
            return Err(RecipeValidationError::NonPositiveQuantity {
                side: RecipeValidationSide::Output,
                item_form_id: item.item_form_id,
                quantity: item.quantity,
            });
        }
        if !available_items.contains(&item.item_form_id) {
            return Err(RecipeValidationError::MissingOutputItem {
                item_form_id: item.item_form_id,
            });
        }
    }

    Ok(())
}
