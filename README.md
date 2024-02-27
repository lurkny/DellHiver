# Welcome to DellHiver

DellHiver is a highly customizable macro program that was designed to work with the game HellDivers 2. Users can quickly add strategems with custom keybinds in the strats.toml file.


# Usage
Simply download the executable or run cargo build --release if you wish to build from source.

## Configuration

Config is done in strats.toml. For example, here would be an example entry to the strats file. 
```
[reinforce]
name = "Reinforce"
combo = ["left", "up", "right", "down"]
keybind = 'p'
```
Currently, the program assumes that you are using WASD to call in strategems, with support for custom stratagem binds coming later. 


