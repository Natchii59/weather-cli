mod utils;
mod weather;

use crate::utils::get_input;
use crate::weather::{display_weather, fetch_weather, handle_error};

use colored::*;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    println!("{}", "Welcome to Weather CLI!".bright_yellow());

    loop {
        let city = get_input("Please enter the name of the city:");
        let country_code = get_input("Please enter the country code (e.g., US for United States):");

        match fetch_weather(&city, &country_code) {
            Ok(response) => display_weather(&response),
            Err(error) => handle_error(&error),
        }

        loop {
            let continue_search =
                get_input("Do you want to search for weather in another city? (yes/no):");

            match continue_search.to_lowercase().as_str() {
                "yes" => break,
                "no" => {
                    println!("{}", "Thank you for using Weather CLI!".bright_red());
                    return;
                }
                _ => continue,
            }
        }
    }
}
