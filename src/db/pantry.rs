// use crate::models::ingredient::{Category, Ingredient, Unit};
use rusqlite::{Connection, Result};

use crate::models::pantry_item::PantryItem;

pub fn add_pantry_item(conn: &Connection, pantry_item: PantryItem) -> Result<()> {

    conn.execute("INSERT INTO pantry (ingredient_id, quantity, expiry_date) values", params)


    Ok(())
}
