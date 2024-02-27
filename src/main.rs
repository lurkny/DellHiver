use rdev::{listen, simulate, EventType, Key};
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;
use std::{fs, thread::sleep};
use toml;

mod key_to_string;
use key_to_string::key_to_string;

#[derive(Debug, Deserialize)]
struct Strategem {
    name: String,
    combo: Vec<String>,
    keybind: String,
}

type Strategems = HashMap<String, Strategem>;

fn main() {
    let mut strategems: HashMap<String, Strategem> = parse_strats_from_file();

    let strategems: HashMap<String, Strategem> = strategems
        .drain()
        .map(|(_, strat)| (strat.keybind.clone(), strat))
        .collect();

    show_welcome(&strategems);

    listen(move |event| {
        if let EventType::KeyPress(key) = event.event_type {
            let key_str = key_to_string(key);
            if let Some(strat) = strategems.get(&key_str) {
                execute_strat(strat);
            }
        }
    })
    .unwrap();
}

fn show_welcome(strats: &Strategems) {
    println!("Welcome to the Strategem Manager!");
    for (key, strat) in strats {
        println!("Press {} to execute {}", key, strat.name);
    }
}

fn execute_strat(strat: &Strategem) {
    // println!("Executing strategem: {}", strat.name);
    for key in strat.combo.clone() {
        sleep(Duration::from_millis(20));
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
    let contents =
        fs::read_to_string("./src/strats.toml").expect("Something went wrong reading the file");

    match toml::from_str(&contents) {
        Ok(strategems) => strategems,
        Err(e) => {
            eprintln!("Failed to parse TOML: {}", e);
            std::process::exit(1);
        }
    }
}
