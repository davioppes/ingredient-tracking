use rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct PantryItem {
    pub id: Option<i64>,
    pub ingredient_id: i64,
    pub name: String,
    pub amount: i64,
    pub expiry_date: String,
}

// impl PantryItem {
//     pub fn create_pantry_item(
//         ingredient_id: i64,
//         // name: String,
//         amount: i64,
//         expiry_date: String,
//     ) -> PantryItem {
//         let new_item = PantryItem {
//             ingredient_id,
//             // name,
//             amount,
//             expiry_date,
//         };

//         new_item
//     }
// }
