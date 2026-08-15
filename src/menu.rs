use crate::models::pantry_item::PantryItem;
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

// INGREDIENT MENU
pub fn ingredients_menu() -> Result<usize, Box<dyn Error>> {
    let ingredient_select = dialoguer::Select::new()
        .with_prompt("What would you like to do?")
        .items(&INGREDIENT_CHOICES)
        .interact()?;
    Ok(ingredient_select)
}

pub fn add_ingredient(conn: &Connection) -> Result<(), Box<dyn Error>> {
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
    Ok(())
}

pub fn remove_ingredient(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let list_ingredients = ingredients::list_all_ingredients(conn)?;

    if list_ingredients.is_empty() {
        println!("There are no ingredients!");
    } else {
        let choice: usize = dialoguer::FuzzySelect::new()
            .with_prompt("Please select an ingredient to remove!")
            .items(&list_ingredients)
            .interact()?;

        let deleted =
            ingredients::remove_ingredient_from_ingredient(conn, &list_ingredients[choice])?;
        if deleted > 0 {
            println!("Ingredient deleted!");
        } else {
            println!("Ingredient not deleted!")
        }
    }

    Ok(())
}

pub fn update_ingredient(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let list_ingredients = ingredients::list_all_ingredients(conn)?;

    if list_ingredients.is_empty() {
        println!("There are no ingredients!");
    } else {
        let choice: usize = dialoguer::FuzzySelect::new()
            .with_prompt("Please select an ingredient to update!")
            .items(&list_ingredients)
            .interact()?;

        println!("Ingredient to update: {}", list_ingredients[choice]);
        let mut new_ingredient = ask_for_ingredient()?;

        new_ingredient.id = list_ingredients[choice].id;

        let confirmation = dialoguer::Confirm::new()
            .with_prompt("Would you like to update?")
            .wait_for_newline(true)
            .default(false)
            .interact()?;

        if confirmation {
            let updated = ingredients::update_ingredient(conn, &new_ingredient)?;
            if updated > 0 {
                println!("Ingredient updated!");
            } else {
                println!("Ingredient not updated!")
            }
        } else {
            println!("Ingredient update aborted!")
        }
    }

    Ok(())
}

// PANTRY MENU
pub fn add_pantry_item(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let list_all_ingredients = ingredients::list_all_ingredients(conn)?;

    if list_all_ingredients.is_empty() {
        println!("No ingredients exist!");
    } else {
        let choice: usize = dialoguer::FuzzySelect::new()
            .with_prompt("Select an ingredient to add to the pantry!")
            .items(&list_all_ingredients)
            .interact()?;

        // name, ingredient_id
        let ingredient_choice: &Ingredient = &list_all_ingredients[choice];

        let amount: f64 = dialoguer::Input::new()
            .with_prompt(format!(
                "Enter amount of {} in {}",
                ingredient_choice.name,
                Ingredient::return_unit_string(&ingredient_choice)
            ))
            .interact_text()?;

        let expiry_day: String = dialoguer::Input::new()
            .with_prompt("Enter day of expiration")
            .validate_with(|input: &String| -> Result<(), String> {})
            .interact_text()?;

        // let new_pantry_item = PantryItem { }
    }

    Ok(())
}

pub fn list_all_ingredients(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let all_ingredients = ingredients::list_all_ingredients(conn)?;
    for ing in all_ingredients {
        println!("{}", ing);
    }

    Ok(())
}
// Helper function to create a new ingredient
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
