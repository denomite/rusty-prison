// CORE FEATURES
// CLI prompts to enter: Prisoner ID, Name, Lastname, Height & Weight, Criminal record
// Save as JSON, load back and display
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct Prisoner {
    id: i32,
    first_name: String,
    last_name: String,
    // height_cm: f64,
    // weight_kg: f64,
    // criminal_record: String,
}

fn read_line(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_names(prompt: &str) -> String {
    loop {
        let input = read_line(prompt);
        let trimmed = input.trim();

        if trimmed.is_empty() {
            println!("input cannot be empty.");
        } else if !trimmed.chars().all(|c| c.is_alphabetic() || c == ' ') {
            println!("Name must contain only letters and spaces.");
        } else {
            return trimmed.to_string();
        }
    }
}

fn input_prisoner() -> Prisoner {
    let id: i32 = loop {
        let input = read_line("Enter prisoner number: ");
        match input.parse::<i32>() {
            Ok(n) => break n,
            Err(_) => println!("Invalid number '{}', try again.", input),
        }
    };

    let first_name = read_names("Enter prisoner name: ");
    let last_name = read_names("Enter prisoner lastname: ");
    Prisoner {
        id,
        first_name,
        last_name,
    }
}
fn main() -> std::io::Result<()> {
    let mut prisoners: Vec<Prisoner> = Vec::new();

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

                let p = input_prisoner();
                prisoners.push(p);
            }
            "2" => {
                for (i, p) in prisoners.iter().enumerate() {
                    println!("\n#{}: {:?}\n", i + 1, p);
                }
            }
            "3" => {
                let json_data = serde_json::to_string(&prisoners).unwrap();
                std::fs::write("prisoners.json", json_data)?;
                println!("Saved to prisoners.json");
            }
            "4" => {
                let file_data = std::fs::read_to_string("prisoners.json")?;
                prisoners = serde_json::from_str(&file_data).unwrap();
                println!("Loaded {} prisoner(s) from file.", prisoners.len());
            }
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }

    Ok(())
}
