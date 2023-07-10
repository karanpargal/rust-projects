use std::io;

fn main() {
    println!("Welcome to the billing system");
    println!("=============================");
    println!("Menu \n1. Press A to place an order \n2. Press B to view products \n3. Press C to exit");
    menu();
}

fn menu(){
    println!("Enter your choice: ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to enter choice");

    let option = choice.trim();

    if option == "A" {
        place_order();
    }
    else if option == "B" {
        view_products();
    }
    else if option == "C" {
        println!("Thank you for using the billing system");
        return;
    }
    else {
        println!("Invalid choice");
        menu();
    }
}

fn place_order(){
    println!("You have placed an order");
}

fn view_products(){
    println!("You have viewed the products");
}
