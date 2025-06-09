mod prisoner;

use prisoner::*;
use std::io;

fn main() -> std::io::Result<()> {
    let mut prisoners: Vec<Prisoner> = Vec::new();

    loop {
        println!("#rusty-prison");
        println!("[1] Add prisoner");
        println!("[2] List prisoners");
        println!("[3] Save to file");
        println!("[4] Load from file");
        println!("[5] Seach prisoner");
        println!("[6] Deleting prisoner");
        println!("[7] Exit");
        println!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => prisoners.push(input_prisoner()),
            "2" => list_prisoners(&prisoners),
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
            "5" => search_prisoner(&prisoners),
            "6" => delete_prisoner(&mut prisoners),
            "7" => break,
            _ => println!("Invalid option, please try again."),
        }
    }

    Ok(())
}
