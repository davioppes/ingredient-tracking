#[derive(Debug, PartialEq, Clone)]
pub enum Category {
    Vegetable,
    Pasta,
    Rice,
    Fruit,
    Sauce,
    Misc,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Grams,
    Ml,
    Number,
}

#[derive(Debug, Clone)]
pub struct Ingredient {
    // pub id: i64,
    pub name: String,
    pub category: Category,
    pub unit: Unit,
}

impl Ingredient {
    pub fn create_ingredient(name: String, category: Category, unit: Unit) -> Ingredient {
        let ingredient = Ingredient {
            name,
            category,
            unit,
        };

        ingredient
    }

    pub fn return_category_string(ingredient: &Ingredient) -> String {
        match ingredient.category {
            Category::Fruit => String::from("Fruit"),
            Category::Vegetable => String::from("Vegetable"),
            Category::Pasta => String::from("Pasta"),
            Category::Rice => String::from("Rice"),
            Category::Sauce => String::from("Sauce"),
            Category::Misc => String::from("Misc"),
        }
    }

    pub fn return_unit_string(ingredient: &Ingredient) -> String {
        match ingredient.unit {
            Unit::Grams => String::from("Grams"),
            Unit::Ml => String::from("Ml"),
            Unit::Number => String::from("Number"),
        }
    }
}
