// CORE FEATURES
// CLI prompts to enter: Prisoner ID, Name, Lastname, Height & Weight, Criminal record
// Save as JSON, load back and display
use std::io;

#[derive(Debug)]
struct Prisoner {
    id: i32,
    first_name: String,
    last_name: String,
    height_cm: f64,
    weight_kg: f64,
    criminal_record: String,
}
fn main() {
    loop {
        println!("#rusty-prison");
        println!("[1] Add prisoner");
        println!("[2] List prisoners");
        println!("[3] Save to file");
        println!("[4] Load from file");
        println!("[5] Exit");

        println!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Adding prisoner");
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}
