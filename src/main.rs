use ingredient_tracking::db;
use ingredient_tracking::menu;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = db::create_connection_path()?;

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
