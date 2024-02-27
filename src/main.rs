use std::{fs, thread::sleep};
use std::collections::HashMap;
use serde::Deserialize;
use toml;
use rdev::{simulate, EventType, Key,listen};
use std::time::Duration;


#[derive(Debug, Deserialize)]
struct Strategem {
    name: String,
    combo: Vec<String>,
}

type Strategems = HashMap<String, Strategem>;

fn main() {
    let strategems: Strategems = parse_strats_from_file();

    listen(move |event| {
        if let EventType::KeyPress(key) = event.event_type {
            /* 
            This code will be garbage until I can add a way for users to specifiy the keybind in config
             */
            if key == Key::KeyP {
                if let Some(strategem) = strategems.get("reinforce") {
                    execute_strat(strategem);
                }
            } 
        }
    }).unwrap();
}

fn execute_strat(strat: &Strategem) {
    println!("Executing strategem: {}", strat.name);
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


