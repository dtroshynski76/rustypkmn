pub struct Arguments {
    file_path: String,
    pub edit_file: bool,
}

const EDIT_OPTION: &str = "--edit";
const HELP_OPTION: &str = "--help";
const VALID_OPTIONS: &[&str] = &[EDIT_OPTION, HELP_OPTION];

impl Arguments {
    pub fn build(args: &[String]) -> Result<Arguments, &'static str> {
        let (options, file_path): (Vec<&String>, Vec<&String>) =
            args.iter().skip(1).partition(|e| e.starts_with("--"));

        if file_path.is_empty() {
            return Err("No file path given");
        } else if file_path.len() > 1 {
            return Err("More than one file path argument given");
        }

        let (valid_options, invalid_options): (Vec<&String>, Vec<&String>) = options
            .iter()
            .partition(|option| VALID_OPTIONS.contains(&option.as_str()));

        if !invalid_options.is_empty() {
            println!("Ignoring invalid options: {:?}", invalid_options);
        }

        if valid_options.contains(&&String::from(HELP_OPTION)) {
            // TODO: print help/usage text
        }

        let path = file_path[0].clone();
        Ok(Arguments {
            file_path: path,
            edit_file: valid_options.contains(&&String::from(EDIT_OPTION)),
        })
    }

    pub fn get_file_path(&self) -> &String {
        &self.file_path
    }
}
