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
    let strategems: Strategems = parse_strats_from_file();

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
    println!("Executing strategem: {}", strat.name);
    for key in strat.combo.clone() {
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

fn parse_strats_from_file() -> Strategems {
    let contents = fs::read_to_string("./src/strats.toml")
        .expect("Something went wrong reading the file");

    match toml::from_str(&contents) {
        Ok(strategems) => strategems,
        Err(e) => {
            eprintln!("Failed to parse TOML: {}", e);
            std::process::exit(1);
        }
    }
}


