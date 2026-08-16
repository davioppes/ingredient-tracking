use core::num;
use std::{error::Error, fmt};

use crate::error::DBErrors::{IngredientInPantry, SqliteError};

#[derive(Debug)]
pub enum DBErrors {
    IngredientInPantry(i64), // Error will contain information of how many pantry items exists for the ingredient
    SqliteError(rusqlite::Error), // A standard sqlite error that will contain the rusqlite error inside
    DuplicateIngredient(String),  // Holds name of ingredient
}

impl Error for DBErrors {}

impl fmt::Display for DBErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DBErrors::IngredientInPantry(number) => write!(
                f,
                "Cannot delete ingredient, there are {} pantry items that reference this ingredient!",
                number.to_string()
            ),
            DBErrors::SqliteError(e) => write!(f, "Database error. Information: {}", e),
            DBErrors::DuplicateIngredient(name) => {
                write!(f, "Ingredient with name '{}' already exists!", name)
            }
        }
    }
}

// For any function that returns DBErrors, we cannot use the ? unless this function is implemented
// Since most of the DB functions will return a rusqlite error, we have to provide a way to convert from a rusqlite error into my error
// We provide an enum, that will hold this rusqlite error
impl From<rusqlite::Error> for DBErrors {
    fn from(value: rusqlite::Error) -> Self {
        DBErrors::SqliteError(value)
    }
}
