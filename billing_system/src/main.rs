use inquire::Select;
mod order;
mod products;

use order::place_order;
use products::view_products;

fn main() {
    println!("Welcome to the billing system");
    println!("=============================");
    menu();
}

fn menu(){

    let options = vec!["Place Order", "See Products", "Exit"];

    let input = Select::new("Menu:", options.clone()).prompt();

    match input {
        Ok(input) => {
            if input == options[0] {
                place_order();
            } else if input == options[1] {
                view_products();
            } else if input == options[2] {
                println!("Thank you for using the billing system");
            }
        },
        Err(_) => {
            println!("Invalid input");
        }
    }
}