use std::env;
use std::fs;
use std::process;

use human_format::{Formatter, Scales};

mod arguments;
use arguments::Arguments;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::build(&args).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let file_path = arguments.get_file_path();

    let contents = load_file_contents(file_path).unwrap_or_else(|err: &str| {
        eprintln!("Problem loading file: {err}");
        process::exit(1);
    });

    let formatted_size = Formatter::new()
        .with_scales(Scales::Binary())
        .format(contents.len() as f64);

    println!("Size of file {file_path}: {}", formatted_size);

    let parsed_contents = save_parser::parse(contents).unwrap_or_else(|err: String| {
        eprintln!("Problem parsing save file contents: {err}");
        process::exit(1);
    });

    // println!("{:#?}", parsed_contents);

    if arguments.edit_file {
        println!("Launching edit GUI...");
        // TODO: launch gui if option set
    }
}

fn load_file_contents(path: &str) -> Result<Vec<u8>, &'static str> {
    let file_exists = match fs::exists(path) {
        Ok(does_exist) => does_exist,
        Err(_) => return Err("Cannot check existence of file"),
    };

    if !file_exists {
        return Err("File does not exist");
    }

    let content = fs::read(path);

    match content {
        Ok(bytes) => Ok(bytes),
        Err(_) => Err("Cannot open file"),
    }
}
