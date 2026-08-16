use std::fmt;

#[derive(Debug, Clone)]
pub struct PantryItem {
    pub id: Option<i64>,
    pub ingredient_id: i64,
    pub name: String,
    pub amount: f64,
    pub expiry_date: String,
}

impl fmt::Display for PantryItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NAME: {} | AMOUNT: {} | EXPIRY_DATE: {} | ID: {} | INGREDIENT ID: {}",
            self.name,
            self.amount.to_string(),
            self.expiry_date,
            self.id.unwrap().to_string(),
            self.ingredient_id.to_string()
        )
    }
}
