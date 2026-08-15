use crate::{
    db::ingredients,
    models::ingredient::{Category, Ingredient, Unit},
};
use rusqlite::{Connection, Error, Result};

pub fn add_ingredient(conn: &Connection, ingredient: &Ingredient) -> Result<usize> {
    let number_row_changed = conn.execute(
        "INSERT INTO ingredients (name,category,unit) values (?1,?2,?3)",
        [
            ingredient.name.clone(),
            Ingredient::return_category_string(&ingredient),
            Ingredient::return_unit_string(&ingredient),
        ],
    )?;

    Ok(number_row_changed)
}

pub fn update_ingredient(conn: &Connection, updated_ingredient: &Ingredient) -> Result<usize> {
    conn.execute(
        "UPDATE ingredients
            SET name = ?1,
            category = ?2,
            unit = ?3
            WHERE id = ?4
    ",
        [
            &updated_ingredient.name,
            &Ingredient::return_category_string(&updated_ingredient),
            &Ingredient::return_unit_string(&updated_ingredient),
            &updated_ingredient.id.unwrap().to_string(),
        ],
    )
}

pub fn list_all_ingredients(conn: &Connection) -> Result<Vec<Ingredient>> {
    //Prepares the SQL statement
    let mut stmt = conn.prepare("SELECT id, name, category, unit FROM ingredients")?;

    //Loops through the query output and transforms each row into a new PantryItem
    let ingredients = stmt
        .query_map([], |row| {
            Ok(Ingredient {
                id: Some(row.get::<_, i64>(0)?),
                name: row.get(1)?,
                category: Ingredient::convert_to_category(row.get(2)?),
                unit: Ingredient::convert_to_unit(row.get(3)?),
            })
        })?
        .collect::<Result<Vec<Ingredient>>>()?; //Collect into a Vector

    Ok(ingredients)
}

pub fn get_ingredient_id(conn: &Connection, ingredient_name: String) -> Result<i64> {
    conn.query_one(
        "SELECT id FROM ingredients WHERE name = ?1",
        [ingredient_name],
        |row| row.get(0),
    )
}

pub fn remove_ingredient_from_ingredient(
    conn: &Connection,
    ingredient: &Ingredient,
) -> Result<usize> {
    // Should always remove as user will select one of the available ingredients
    let number_removed = conn.execute(
        "DELETE FROM ingredients WHERE name = ?1",
        [ingredient.name.clone()],
    )?;

    Ok(number_removed)
}
// Used for testing
// In actual program, user will select an Ingredient struct item to remove from a list of all ingredient (or searched ingredients)
pub fn remove_ingredient_from_name(conn: &Connection, name: String) -> Result<()> {
    conn.execute("DELETE FROM ingredients WHERE name = ?1", [name.clone()])?;

    Ok(())
}
