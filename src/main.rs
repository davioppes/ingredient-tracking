use ingredient_tracking::db;
use ingredient_tracking::menu;
use std::error::Error;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    clear_screen();
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
                    "Exit" => exit = true,
                    _ => unreachable!("Select failed"),
                }

                clear_screen();
            }
            "ingredient" => {
                state = "ingredient";
                
                let choice = menu::ingredients_menu()?;

                match menu::INGREDIENT_CHOICES[choice] {
                    "Add Ingredient" => {
                        menu::add_ingredient(&conn)?;

                        clear_screen();
                    }
                    "Remove Ingredient" => {
                        menu::remove_ingredient(&conn)?;

                        clear_screen();
                    }
                    "Update Ingredient" => {
                        menu::update_ingredient(&conn)?;

                        clear_screen();
                    }
                    "List All Ingredients" => {
                        menu::list_all_ingredients(&conn)?;

                        clear_screen();
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

                        clear_screen();
                    }
                    "Remove Pantry Item" => {
                        menu::remove_pantry_item(&conn)?;

                        clear_screen();
                    }
                    "Update Pantry Item" => {
                        menu::update_pantry_item(&conn)?;

                        clear_screen();
                    }
                    "List Pantry" => {
                        menu::list_all_pantry_items(&conn)?;

                        clear_screen();
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

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}