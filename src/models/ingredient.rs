use std::fmt;

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
    pub id: Option<i64>,
    pub name: String,
    pub category: Category,
    pub unit: Unit,
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {} | NAME: {} | CATEGORY: {} | UNIT: {}",
            self.id.unwrap().to_string(),
            self.name,
            Ingredient::return_category_string(self),
            Ingredient::return_unit_string(self)
        )
    }
}

impl Ingredient {
    pub fn convert_to_category(ingredient: String) -> Category {
        match ingredient.as_str() {
            "Vegetable" => Category::Vegetable,
            "Fruit" => Category::Fruit,
            "Pasta" => Category::Pasta,
            "Rice" => Category::Rice,
            "Sauce" => Category::Sauce,
            "Misc" => Category::Misc,
            _ => Category::Misc,
        }
    }

    pub fn convert_to_unit(unit: String) -> Unit {
        match unit.as_str() {
            "Grams" => Unit::Grams,
            "Ml" => Unit::Ml,
            "Number" => Unit::Number,
            _ => panic!("Unit is not correct"),
        }
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
