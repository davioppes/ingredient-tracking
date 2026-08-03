use std::error::Error;

use ingredient_tracking::db;
use ingredient_tracking::db::ingredients;
use ingredient_tracking::models::ingredient::{Category, Ingredient, Unit};

fn main() -> Result<(), Box<dyn Error>> {
    let conn = db::create_connection_memory()?;

    let new_ingredient: Ingredient =
        Ingredient::create_ingredient(String::from("Potato"), Category::Vegetable, Unit::Number);

    ingredients::add_ingredient(&conn, &new_ingredient)?;
    pretty_sqlite::print_table(&conn, "ingredients")?;

    ingredients::remove_ingredient_from_name(&conn, String::from("Tomato"))?;
    pretty_sqlite::print_table(&conn, "ingredients")?;

    Ok(())
}
