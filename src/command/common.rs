use libbdgt::error::{Result, Error};
use libbdgt::storage::CategoryType;

use crate::errors;
use crate::misc;


/// List of all possible category types.
pub(crate) fn category_types() -> Vec<&'static str> {
    vec!["Income", "Outcome"]
}


/// Returns a category type by its corresponding index.
/// 
/// * `idx` - index to return category type for
pub(crate) fn category_type_by_index(idx: usize) -> Result<CategoryType> {
    match idx {
        0 | 1 => Ok(misc::either!(idx == 0 => CategoryType::Income; CategoryType::Outcome)),
        _     => Err(Error::from_message_with_extra(errors::VALUE_OUT_OF_RANGE, format!("index: {}", idx)))
    }
}


/// Converts category type into a string.
/// 
/// * `category_type` - type to convert
pub(crate) fn category_type_to_string(category_type: CategoryType) -> String {
    match category_type {
        CategoryType::Income  => "Income".to_owned(),
        CategoryType::Outcome => "Outcome".to_owned(),
    }
}