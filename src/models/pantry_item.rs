use rusqlite::Connection;

pub struct PantryItem {
    ingredient_id: i64,
    name: String,
    amount: i64,
    expiry_date: String,
}

impl PantryItem {
    pub fn create_pantry_item(
        ingredient_id: i64,
        name: String,
        amount: i64,
        expiry_date: String,
    ) -> PantryItem {
        let new_item = PantryItem {
            ingredient_id,
            name,
            amount,
            expiry_date,
        };

        new_item
    }
}
