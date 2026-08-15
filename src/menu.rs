use crate::{db::ingredients, models::ingredient::Ingredient};
use dialoguer;
use rusqlite::Connection;
use rusqlite::Error as SQLError;
use rusqlite::ffi;
use std::error::Error;

// STARTING MENU CHOICES
pub const STARTING_CHOICES: [&str; 4] = ["Ingredients", "Pantry", "Recipes", "Exit"];

// INGREDIENT MENU CHOICE SELECTIONS
pub const INGREDIENT_CHOICES: [&str; 5] = [
    "Add Ingredient",
    "Remove Ingredient",
    "Update Ingredient",
    "List All Ingredients",
    "Go Back",
];

const CATEGORIES: [&str; 6] = ["Vegetable", "Fruit", "Pasta", "Rice", "Sauce", "Misc"];

const UNITS: [&str; 3] = ["Grams", "Ml", "Number"];

// PANTRY MENU CHOICES
pub const PANTRY_CHOICES: [&str; 4] = [
    "Add Pantry Item",
    "Remove Pantry Item",
    "List Pantry",
    "Go Back",
];

pub fn main_menu() -> Result<usize, Box<dyn Error>> {
    let select = dialoguer::Select::new()
        .with_prompt("What would you like to do?")
        .items(&STARTING_CHOICES)
        .interact()?;

    Ok(select)
}

pub fn ingredients_menu(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let ingredient_select = dialoguer::Select::new()
        .with_prompt("What would you like to do?")
        .items(&INGREDIENT_CHOICES)
        .interact()?;

    match INGREDIENT_CHOICES[ingredient_select] {
        "Add Ingredient" => {
            let new_ingredient = ask_for_ingredient()?;
            // Check for any errors and add ingredient
            match ingredients::add_ingredient(conn, &new_ingredient) {
                Ok(number_row) => {
                    if number_row == 0 {
                        println!("Ingredient was not added.")
                    }
                }
                Err(SQLError::SqliteFailure(err, _)) => match err.extended_code {
                    ffi::SQLITE_CONSTRAINT_UNIQUE => {
                        println!("Ingredient name already exists!")
                    }
                    _ => println!("Another constraint error has occured!"),
                },
                Err(error) => {
                    println!("Error has occured. {:?}", error)
                }
            };
        }
        "Remove Ingredient" => {}
        "Update Ingredient" => {}
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

fn ask_for_ingredient() -> Result<Ingredient, Box<dyn Error>> {
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
        .items(&CATEGORIES)
        .interact()?;
    let unit = dialoguer::Select::new()
        .with_prompt("What is the unit?")
        .items(&UNITS)
        .interact()?;

    // Create new ingredient to be passed into the add_ingredient function
    let new_ingredient = Ingredient {
        id: None,
        name: name,
        category: Ingredient::convert_to_category(CATEGORIES[category].to_string()),
        unit: Ingredient::convert_to_unit(UNITS[unit].to_string()),
    };

    Ok(new_ingredient)
}
