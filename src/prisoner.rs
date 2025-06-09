use serde::{Deserialize, Serialize};
use std::io::{self, BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub struct Prisoner {
    id: i32,
    first_name: String,
    last_name: String,
    height_cm: f64,
    weight_kg: f64,
    criminal_record: String,
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

fn load_prisoners_from_file(path: &str) -> Vec<Prisoner> {
    if let Ok(file) = std::fs::File::open(path) {
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_default()
    } else {
        vec![]
    }
}

fn id_exists(id: i32, prisoners: &[Prisoner]) -> bool {
    prisoners.iter().any(|p| p.id == id)
}

pub fn input_prisoner() -> Prisoner {
    let path = "prisoners.json";
    let prisoners = load_prisoners_from_file(path);

    let id: i32 = loop {
        let input = read_line("Enter prisoner number: ");
        match input.trim().parse::<i32>() {
            Ok(n) => {
                if id_exists(n, &prisoners) {
                    println!("ID {} already exist. Try another.", n);
                } else {
                    break n;
                }
            }
            Err(_) => println!("Invalid number '{}', try again.", input.trim()),
        }
    };

    let first_name = read_names("Enter prisoner name: ");
    let last_name = read_names("Enter prisoner lastname: ");

    let height_cm = loop {
        let input = read_line("Enter height (cm): ");
        if let Ok(h) = input.parse::<f64>() {
            break h;
        } else {
            println!("Invalid height, try again.");
        }
    };

    let weight_kg = loop {
        let input = read_line("Enter weight (kg): ");
        if let Ok(w) = input.parse::<f64>() {
            break w;
        } else {
            println!("Invalid weight, try again.");
        }
    };

    let criminal_record = read_line("Enter criminal record description: ");

    Prisoner {
        id,
        first_name,
        last_name,
        height_cm,
        weight_kg,
        criminal_record,
    }
}

pub fn search_prisoner(prisoners: &[Prisoner]) {
    let query = read_line("Enter prisoner ID or name to search: ")
        .trim()
        .to_lowercase();

    let mut found = false;
    for p in prisoners {
        if p.id.to_string() == query || p.first_name.to_lowercase().contains(&query) {
            println!("🔍 Found prisoner: ");
            println!(
                "ID: {}, Name: {} {}, Height: {}, Weight: {}, Criminal record: {}",
                p.id, p.first_name, p.last_name, p.height_cm, p.height_cm, p.criminal_record
            );

            found = true;
        }

        if !found {
            println!("❌ No matching prisoner found.");
        }
    }
}

pub fn delete_prisoner(prisoners: &mut Vec<Prisoner>) {
    let input = read_line("Enter the ID of the prisoner to delete: ");

    match input.trim().parse::<i32>() {
        Ok(id) => {
            let index = prisoners.iter().position(|p| p.id == id);

            match index {
                Some(i) => {
                    let p = &prisoners[i];
                    println!(
                        "🗑️ Deleting prisoner: {} {}, ID: {}",
                        p.first_name, p.last_name, p.id
                    );
                    prisoners.remove(i);
                    println!("✅ Prisoner deleted.");
                }
                None => println!("❌ No prisoner found with ID {}", id),
            }
        }
        Err(_) => println!("⚠️ Invalid ID input."),
    }
}

pub fn list_prisoners(prisoners: &[Prisoner]) {
    for (i, p) in prisoners.iter().enumerate() {
        println!("\n#{}: {:?}\n", i + 1, p);
    }
}
