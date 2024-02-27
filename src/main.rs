use std::fs;
use std::collections::HashMap;
use serde::Deserialize;
use toml;
use rdev::{simulate, EventType, Key};


#[derive(Deserialize, Debug)]
struct Strategem {
    name: String,
    combo: Vec<String>,
}

type Strategems = HashMap<String, Strategem>;

fn main() {
    let contents = fs::read_to_string("./src/strats.toml")
        .expect("Something went wrong reading the file");

    let strategems: Strategems = match toml::from_str(&contents) {
        Ok(strategems) => strategems,
        Err(e) => {
            eprintln!("Failed to parse TOML: {}", e);
            return;
        }
    };

    
    loop {
        let mut input = String::new();
        println!("Enter the name of the strategem you want to execute: ");
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match strategems.get(input) {
            Some(strat) => execute_strat(strat),
            None => println!("No strategem found with that name"),
        }
    }
}

fn execute_strat(strat: &Strategem) {
    for key in strat.combo.clone() {
        //match left to A and right to D and W to up and S to down
        match key.as_str() {
            "left" => send(&EventType::KeyPress(Key::KeyA)),
            "right" => send(&EventType::KeyPress(Key::KeyD)),
            "up" => send(&EventType::KeyPress(Key::KeyW)),
            "down" => send(&EventType::KeyPress(Key::KeyS)),
            _ => (),
           
        }
    }
}

fn send(event: &EventType) {
     match simulate(event) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to simulate event: {}", e),
     }
}


