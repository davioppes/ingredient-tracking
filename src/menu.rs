use crate::db::pantry;
use crate::error::DBErrors;
use crate::models::pantry_item::PantryItem;
use crate::{db::ingredients, models::ingredient::Ingredient};
use chrono::{self, NaiveDate};
use dialoguer;
use dialoguer::theme::ColorfulTheme;
use rusqlite::Connection;
use std::error::Error;
use std::io::Write;

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
pub const PANTRY_CHOICES: [&str; 5] = [
    "Add Pantry Item",
    "Remove Pantry Item",
    "Update Pantry Item",
    "List Pantry",
    "Go Back",
];

pub fn main_menu() -> Result<usize, Box<dyn Error>> {
    let select = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&STARTING_CHOICES)
        .interact()?;

    Ok(select)
}

// INGREDIENT MENU
pub fn ingredients_menu() -> Result<usize, Box<dyn Error>> {
    let ingredient_select = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&INGREDIENT_CHOICES)
        .interact()?;
    Ok(ingredient_select)
}

pub fn add_ingredient(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let new_ingredient = ask_for_ingredient()?;
    // Check for any errors and add ingredient
    match ingredients::add_ingredient(conn, &new_ingredient) {
        Ok(_) => {
            println!("Ingredient was added.");
        }
        Err(DBErrors::DuplicateIngredient(name)) => {
            println!("{name} already exists! Try another name.");
        }
        Err(error) => {
            println!("Error has occured. {}", error);
        }
    }

    Ok(())
}

pub fn remove_ingredient(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let list_ingredients = ingredients::list_all_ingredients(conn)?;

    if list_ingredients.is_empty() {
        println!("There are no ingredients!");
        return Ok(());
    }

    let choice: usize = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select an ingredient to remove!")
        .items(&list_ingredients)
        .interact()?;

    match ingredients::remove_ingredient_from_ingredient(conn, &list_ingredients[choice]) {
        Ok(_) => println!("Ingredient deleted"),
        Err(DBErrors::IngredientInPantry(number)) => {
            println!("There are {number} pantry items that reference this ingredient!")
        }
        Err(e) => println!("Error has occured. {}", e),
    }

    Ok(())
}

pub fn update_ingredient(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let list_ingredients = ingredients::list_all_ingredients(conn)?;

    if list_ingredients.is_empty() {
        println!("There are no ingredients!");
        return Ok(());
    }

    let choice: usize = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select an ingredient to update!")
        .items(&list_ingredients)
        .interact()?;

    println!("Ingredient to update: {}", list_ingredients[choice]);
    let mut new_ingredient = ask_for_ingredient()?;

    new_ingredient.id = list_ingredients[choice].id;

    let confirmation = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to update?")
        .wait_for_newline(true)
        .default(false)
        .interact()?;

    if confirmation {
        match ingredients::update_ingredient(conn, &new_ingredient) {
            Ok(_) => println!("Ingredient was updated successfully!"),
            Err(DBErrors::DuplicateIngredient(name)) => {
                println!("Cannot update ingredient as {name} already exists!")
            }
            Err(e) => println!("Error has occured. {}", e),
        };
    } else {
        println!("Ingredient update aborted!")
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

// PANTRY MENU
pub fn pantry_menu() -> Result<usize, Box<dyn Error>> {
    let pantry_select = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&PANTRY_CHOICES)
        .interact()?;
    Ok(pantry_select)
}

pub fn add_pantry_item(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let list_all_ingredients = ingredients::list_all_ingredients(conn)?;

    if list_all_ingredients.is_empty() {
        println!("No ingredients exist!");
    } else {
        let choice: usize = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an ingredient to add to the pantry!")
            .items(&list_all_ingredients)
            .interact()?;

        // name, ingredient_id
        let ingredient_choice: &Ingredient = &list_all_ingredients[choice];

        let amount: f64 = dialoguer::Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Enter amount of {} in {}",
                ingredient_choice.name,
                Ingredient::return_unit_string(&ingredient_choice)
            ))
            .interact_text()?;

        let current_date = chrono::Local::now().date_naive();

        let expiry_date_string: String = dialoguer::Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter day of expiration as YYYY-MM-DD:")
            .validate_with(|input: &String| -> Result<(), String> {
                match NaiveDate::parse_from_str(input, "%Y-%m-%d") {
                    Ok(date) if date >= current_date => Ok(()),
                    Ok(_) => Err("Date cannot be in the past! Enter new date!".to_string()),
                    Err(_) => Err("Enter in the format YYYY-MM-DD".to_string()),
                }
            })
            .interact_text()?;

        let expiry_date = NaiveDate::parse_from_str(&expiry_date_string, "%Y-%m-%d").unwrap();

        let new_pantry_item: PantryItem = PantryItem {
            id: None,
            ingredient_id: ingredient_choice.id.unwrap(),
            name: ingredient_choice.name.clone(),
            amount: amount,
            expiry_date: expiry_date.to_string(),
        };

        pantry::add_pantry_item(conn, &new_pantry_item)?;

        // let new_pantry_item = PantryItem { }
    }

    Ok(())
}

pub fn remove_pantry_item(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let all_pantry_items = pantry::list_pantry_items_all(conn)?;

    if all_pantry_items.is_empty() {
        println!("There are no pantry items to remove!");
        return Ok(());
    }

    let choice: usize = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select an item to remove:")
        .items(&all_pantry_items)
        .interact()?;

    match pantry::remove_pantry_item(conn, all_pantry_items[choice].id.unwrap()) {
        Ok(_) => println!("Pantry item removed"),
        Err(e) => println!("Error has occured. {}", e),
    }

    Ok(())
}

pub fn update_pantry_item(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let all_pantry_items = pantry::list_pantry_items_all(conn)?;

    if all_pantry_items.is_empty() {
        println!("There are no pantry items to remove!");
        return Ok(());
    }

    let choice: usize = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select an item to update:")
        .items(&all_pantry_items)
        .interact()?;

    let current_item = &all_pantry_items[choice];

    println!("Ingredient to update: {}", current_item);

    let amount: f64 = dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Enter updated amount or press enter to not change",))
        .allow_empty(true)
        .default(current_item.amount)
        .show_default(true)
        .interact_text()?;

    let current_date = chrono::Local::now().date_naive();

    let expiry_date_string: String = dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter new date as YYYY-MM-DD or press enter to not change")
        .validate_with(|input: &String| -> Result<(), String> {
            match NaiveDate::parse_from_str(input, "%Y-%m-%d") {
                Ok(date) if date >= current_date => Ok(()),
                Ok(_) => Err("Date cannot be in the past! Enter new date!".to_string()),
                Err(_) => Err("Enter in the format YYYY-MM-DD".to_string()),
            }
        })
        .allow_empty(true)
        .default(current_item.expiry_date.clone())
        .show_default(true)
        .interact_text()?;

    let expiry_date = NaiveDate::parse_from_str(&expiry_date_string, "%Y-%m-%d").unwrap();

    let updated_item: PantryItem = PantryItem {
        id: current_item.id,
        ingredient_id: current_item.ingredient_id,
        name: current_item.name.clone(),
        amount: amount,
        expiry_date: expiry_date.to_string(),
    };

    let confirmation = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to update?")
        .wait_for_newline(true)
        .default(false)
        .interact()?;

    if confirmation {
        match pantry::update_pantry_item(conn, &updated_item) {
            Ok(_) => println!("Item was updated successfully!"),
            Err(e) => println!("Error has occured. {}", e),
        };
    } else {
        println!("Item update aborted!")
    }

    Ok(())
}

pub fn list_all_pantry_items(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let items = pantry::list_pantry_items_all(conn)?;
    for item in items {
        println!("{}", item);
    }

    Ok(())
}
// Helper function to create a new ingredient
fn ask_for_ingredient() -> Result<Ingredient, Box<dyn Error>> {
    let name = dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Name of ingredient?")
        .validate_with(|input: &String| -> Result<(), String> {
            if input.is_empty() {
                Err("Name cannot be empty".into())
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    let category = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What is the category?")
        .items(&CATEGORIES)
        .interact()?;
    let unit = dialoguer::Select::with_theme(&ColorfulTheme::default())
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

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}