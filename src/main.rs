mod prisoner;

use prisoner::*;
use std::io;
use std::io::{Write, stdout};

fn main() -> std::io::Result<()> {
    let mut prisoners: Vec<Prisoner> = Vec::new();

    loop {
        println!("\n");
        println!("Rusty Prsion");
        println!("\n");
        println!("[1] Add prisoner");
        println!("[2] List prisoners");
        println!("[3] Save to file");
        println!("[4] Load from file");
        println!("[5] Seach prisoner");
        println!("[6] Deleting prisoner");
        println!("[7] Exit");
        println!("\n");

        print!("Enter your choice: ");
        stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => prisoners.push(input_prisoner()),
            "2" => list_prisoners(&prisoners),
            "3" => save_prisoner(&prisoners),
            "4" => search_prisoner(&prisoners),
            "5" => delete_prisoner(&mut prisoners),
            "6" => break,
            _ => println!("Invalid option, please try again."),
        }
    }

    Ok(())
}
