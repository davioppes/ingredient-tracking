// use dialoguer;
// use rusqlite::Connection;
// use rusqlite::ErrorCode;
// use rusqlite::ffi;
// use std::error::Error;
// use std::vec;

// use ingredient_tracking::db;
// use ingredient_tracking::db::ingredients;
// use ingredient_tracking::db::pantry;
// use ingredient_tracking::models::ingredient::{Category, Ingredient, Unit};
// use ingredient_tracking::models::pantry_item::PantryItem;
// use rusqlite::Error as SQLError;

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

use ingredient_tracking::db;
use ingredient_tracking::menu;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = db::create_connection_memory()?;

    let mut exit: bool = false;
    let mut state: &str = "menu";

    while !exit {
        match state {
            "menu" => {
                state = "menu";
                let choice = menu::main_menu()?;

                match menu::STARTING_CHOICES[choice] {
                    "Ingredients" => state = "ingredient",
                    "Pantry" => state = "pantry",
                    "Recipes" => state = "recipes",
                    "Exit" => exit = true,
                    _ => unreachable!("Select failed"),
                }
            }
            "ingredient" => {
                state = "ingredient";
                let choice = menu::ingredients_menu()?;

                match menu::INGREDIENT_CHOICES[choice] {
                    "Add Ingredient" => {
                        menu::add_ingredient(&conn)?;
                    }
                    "Remove Ingredient" => {
                        menu::remove_ingredient(&conn)?;
                    }
                    "Update Ingredient" => {
                        menu::update_ingredient(&conn)?;
                    }
                    "List All Ingredients" => {
                        menu::list_all_ingredients(&conn)?;
                    }
                    "Go Back" => state = "menu",
                    _ => unreachable!(),
                }
            }
            "pantry" => {
                state = "pantry";
                let choice = menu::pantry_menu()?;

                match menu::PANTRY_CHOICES[choice] {
                    "Add Pantry Item" => {
                        menu::add_pantry_item(&conn)?;
                    }
                    "Remove Pantry Item" => {
                        menu::remove_pantry_item(&conn)?;
                    }
                    "Update Pantry Item" => {
                        menu::update_pantry_item(&conn)?;
                    }
                    "List Pantry" => {
                        menu::list_all_pantry_items(&conn)?;
                    }
                    "Go Back" => state = "menu",
                    _ => unreachable!(),
                }
            }
            "recipe" => {}
            _ => unreachable!("Menu match failed!"),
        }
    }

    Ok(())
}
