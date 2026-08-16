// use clap::error::Result;
// use crate::models::ingredient::{Category, Ingredient, Unit};
use rusqlite::{Connection, Result};

use crate::models::pantry_item::PantryItem;

/*
PANTRY TABLE:
- id
- ingredient_id
- quantity
- expiry_date
*/

pub fn add_pantry_item(conn: &Connection, pantry_item: &PantryItem) -> Result<()> {
    conn.execute(
        "INSERT INTO pantry (ingredient_id, quantity, expiry_date) values (?1,?2,?3)",
        [
            pantry_item.ingredient_id.to_string(),
            pantry_item.amount.to_string(),
            pantry_item.expiry_date.clone(),
        ],
    )?;

    Ok(())
}

pub fn update_pantry_item(conn: &Connection, updated_pantry_item: &PantryItem) -> Result<usize> {
    conn.execute(
        "UPDATE pantry
            SET quantity = ?1,
            expiry_date = ?2
            WHERE id = ?3
    ",
        [
            &updated_pantry_item.amount.to_string(),
            &updated_pantry_item.expiry_date.to_string(),
            &updated_pantry_item.id.unwrap().to_string(),
        ],
    )
}

pub fn list_pantry_items_all(conn: &Connection) -> Result<Vec<PantryItem>> {
    //Prepares the SQL statement
    let mut stmt = conn.prepare(
        "SELECT pantry.id, pantry.ingredient_id, ingredients.name, pantry.quantity, pantry.expiry_date 
            FROM pantry 
            INNER JOIN ingredients ON ingredients.id = pantry.ingredient_id",
    )?;

    //Loops through the query output and transforms each row into a new PantryItem
    let items = stmt
        .query_map([], |row| {
            Ok(PantryItem {
                id: Some(row.get::<_, i64>(0)?),
                ingredient_id: row.get::<_, i64>(1)?,
                name: row.get::<_, String>(2)?,
                amount: row.get::<_, f64>(3)?,
                expiry_date: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<PantryItem>>>()?; //Collect into a Vector

    Ok(items)
}

pub fn list_pantry_items_specific(
    conn: &Connection,
    ingredient_id: i64,
) -> Result<Vec<PantryItem>> {
    //Prepares the SQL statement
    let mut stmt = conn.prepare(
        "SELECT pantry.id, ingredients.name, pantry.quantity, pantry.expiry_date 
            FROM pantry 
            INNER JOIN ingredients ON ingredients.id = pantry.ingredient_id
            WHERE pantry.ingredient_id = (?1)",
    )?;

    //Loops through the query output and transforms each row into a new PantryItem
    let items = stmt
        .query_map([ingredient_id], |row| {
            Ok(PantryItem {
                id: Some(row.get::<_, i64>(0)?),
                ingredient_id: ingredient_id,
                name: row.get::<_, String>(1)?,
                amount: row.get::<_, f64>(2)?,
                expiry_date: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<PantryItem>>>()?; //Collect into a Vector

    Ok(items)
}

pub fn remove_pantry_item(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM pantry WHERE id = ?1", [id])?;

    Ok(())
}
