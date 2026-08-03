use crate::models::ingredient::{Category, Ingredient, Unit};
use rusqlite::{Connection, Result};

pub fn add_ingredient(conn: &Connection, ingredient: &Ingredient) -> Result<()> {
    conn.execute(
        "INSERT INTO ingredients (name,category,unit) values (?1,?2,?3)",
        [
            ingredient.name.clone(),
            Ingredient::return_category_string(&ingredient),
            Ingredient::return_unit_string(&ingredient),
        ],
    )?;

    Ok(())
}

pub fn remove_ingredient_from_ingredient(conn: &Connection, ingredient: &Ingredient) -> Result<()> {
    conn.execute(
        "DELETE FROM ingredients WHERE name = ?1",
        [ingredient.name.clone()],
    )?;

    Ok(())
}

pub fn remove_ingredient_from_name(conn: &Connection, name: String) -> Result<()> {
    conn.execute("DELETE FROM ingredients WHERE name = ?1", [name.clone()])?;

    Ok(())
}


