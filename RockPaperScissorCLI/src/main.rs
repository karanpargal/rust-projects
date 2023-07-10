use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    println!("Rock Paper Scissors");
    println!("====================");
    println!("Enter your choice: (R)ock, (P)aper, (S)cissors");

    let mut choice = String::new();

    let choices = [String::from("R"), String::from("P"), String::from("S")];
    let mut rng = thread_rng();
    let ai_choice = choices.choose(&mut rng).expect("Failed to choose");

    io::stdin().read_line(&mut choice).expect("Failed to enter choice");

    if choice.trim() == ai_choice {
        println!("Draw");
    }
    else if choice.trim() == "R" && ai_choice == "S" {
        println!("You win");
    }
    else if choice.trim() == "P" && ai_choice == "R" {
        println!("You win");
    }
    else if choice.trim() == "S" && ai_choice == "P" {
        println!("You win");
    }
    else {
        println!("You lose");
    }
}
