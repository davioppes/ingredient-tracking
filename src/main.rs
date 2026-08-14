use dialoguer;
use rusqlite::Connection;
use std::error::Error;
use std::vec;

use ingredient_tracking::db;
use ingredient_tracking::db::ingredients;
use ingredient_tracking::db::pantry;
use ingredient_tracking::models::ingredient::{Category, Ingredient, Unit};
use ingredient_tracking::models::pantry_item::PantryItem;
use rusqlite::Error as SQLError;

// fn main() -> Result<(), Box<dyn Error>> {
//     let conn = db::create_connection_memory()?;

//     let new_ingredient: Ingredient = Ingredient {
//         name: String::from("Potato"),
//         category: Category::Vegetable,
//         unit: Unit::Number,
//     };

//     ingredients::add_ingredient(&conn, &new_ingredient)?;
//     pretty_sqlite::print_table(&conn, "ingredients")?;

//     // ingredients::remove_ingredient_from_name(&conn, String::from("Tomato"))?;
//     // pretty_sqlite::print_table(&conn, "ingredients")?;

//     let tomato_id = match ingredients::get_ingredient_id(&conn, String::from("Hello")) {
//         Ok(id) => id,
//         Err(SQLError::QueryReturnedNoRows) => {
//             println!("Ingredient does not exist!");
//             -1
//         }
//         Err(error) => {
//             println!("Unexpected error: {:?}", error);
//             -1
//         }
//     };

//     let new_pantry_item: PantryItem = PantryItem {
//         id: None,
//         ingredient_id: tomato_id,
//         amount: 3,
//         expiry_date: String::from("16/04/2006"),
//     };

//     println!("Printing Pantry Items");

//     pantry::add_pantry_item(&conn, &new_pantry_item)?;
//     pantry::add_pantry_item(&conn, &new_pantry_item)?;
//     pantry::add_pantry_item(&conn, &new_pantry_item)?;
//     pantry::add_pantry_item(&conn, &new_pantry_item)?;

//     pretty_sqlite::print_table(&conn, "pantry")?;

//     let items = pantry::list_pantry_items(&conn, tomato_id)?;

//     for item in items {
//         println!("{:?}", item);
//     }

//     pantry::remove_pantry_item(&conn, 3)?;
//     pretty_sqlite::print_table(&conn, "pantry")?;

//     Ok(())
// }

fn main() -> Result<(), Box<dyn Error>> {
    let conn = db::create_connection_memory()?;

    let mut exit: bool = false;
    let starting_choices: Vec<&str> = vec!["Ingredients", "Pantry", "Recipes", "Exit"];
    let pantry_choices: Vec<&str> = vec![
        "Add Pantry Item",
        "Remove Pantry Item",
        "List Pantry",
        "Go Back",
    ];

    while !exit {
        let select = dialoguer::Select::new()
            .with_prompt("What would you like to do?")
            .items(&starting_choices)
            .interact()?;

        match starting_choices[select] {
            "Ingredients" => ingredients_menu(&conn)?,
            "Pantry" => {}
            "Recipes" => {}
            "Exit" => exit = true,
            _ => panic!("Select failed"),
        }
    }

    Ok(())
}

fn ingredients_menu(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let ingredient_choices: Vec<&str> = vec![
        "Add Ingredient",
        "Remove Ingredient",
        "List All Ingredients",
        "Go Back",
    ];

    let categories: Vec<&str> = vec!["Vegetable", "Fruit", "Pasta", "Rice", "Sauce", "Misc"];

    let units: Vec<&str> = vec!["Grams", "Ml", "Number"];

    let ingredient_select = dialoguer::Select::new()
        .with_prompt("What would you like to do?")
        .items(&ingredient_choices)
        .interact()?;

    match ingredient_choices[ingredient_select] {
        "Add Ingredient" => {
            let name = dialoguer::Input::new()
                .with_prompt("Name of ingredient?")
                .validate_with(|input: &String| -> Result<(), String> {
                    if input.is_empty() {
                        Err("Name cannot be empty".into())
                    } else {
                        Ok(())
                    }
                })
                .interact_text()?;

            let category = dialoguer::Select::new()
                .with_prompt("What is the category?")
                .items(&categories)
                .interact()?;
            let unit = dialoguer::Select::new()
                .with_prompt("What is the unit?")
                .items(&units)
                .interact()?;

            let new_ingredient = Ingredient {
                name: name,
                category: Ingredient::convert_to_category(categories[category].to_string()),
                unit: Ingredient::convert_to_unit(units[unit].to_string()),
            };

            ingredients::add_ingredient(conn, &new_ingredient);
        }
        "Remove Ingredient" => {}
        "List All Ingredients" => {
            let all_ingredients = ingredients::list_all_ingredients(conn)?;
            for ing in all_ingredients {
                println!("{:?}", ing);
            }
        }
        "Go Back" => {}
        _ => unreachable!(),
    }
    Ok(())
}
