# Overview

This is intended to be a parser and save editor for Pokemon save files written in Rust.

`save_parser`

- parses Pokemon save files into a common format suitable for editing
- takes the common format and resolves it to a new save file
- can print out the contents of the save file to stdout in a human readable way

`save_editor`

- exposes a GUI for editing Pokemon save files

# Resources I used

- [Bulbapedia save data structure](<https://bulbapedia.bulbagarden.net/wiki/Save_data_structure_(Generation_III)>)
