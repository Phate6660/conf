use std::io::{Error, Write};

/// Creates the specified default configuration at the specified path when called.
fn write_default_config(config: &str, config_path: &str) {
    let mut file = match std::fs::File::create(config_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Could not create config file because: '{}'.", e);
            return;
        }
    };
    writeln!(file, "{}", config).unwrap();
}

/// Returns a vector of `String`s encased in a `Result`, each of which is a line of the file.
fn parse_file(path: &str, config: &str) -> Result<Vec<String>, Error> {
    let contents = sflib::read(path);
    if let Ok(contents) = contents {
        let lines = contents.lines();
        let mut result = Vec::new();
        for line in lines {
            result.push(line.to_string());
        }
        Ok(result)
    } else {
        // If the config doesn't exist, write a default one.
        write_default_config(config, path);
        println!("Created a default config at '{}'.", path);
        // Go back to the start of the function.
        parse_file(path, config)
    }
}

/// Returns a vector of tuple of `String`s, encased in a `Result` and returned.
/// The first tuple element is the option name, and the second is the option value.
/// 
/// # Errors 
/// 
/// If `sflib::read()` or `parse_file()` returns an error, the error is returned.
/// Otherwise if there is an error in this function, the error is printed and an empty vector is returned.
pub fn get_options(path: &str, config: &str) -> Result<Vec<(String, String)>, Error> {
    let mut options = Vec::new();
    let lines = parse_file(path, config);
    match lines {
        Ok(valid_lines) => {
            for line in valid_lines {
                // Ignore comment lines.
                if line.starts_with("#") {
                    continue;
                }
                let line_vec: Vec<&str> = line.split(|c| c == '=').collect();
                let option = line_vec[0];
                let raw_value = line_vec[1].to_string();
                // Remove any surrounding quotes from the value.
                let value = raw_value.trim_matches('"');
                options.push((option.to_string(), value.to_string()));
            }
        },
        Err(e) => {
            println!("[ERROR]: {}", e);
        }
    }
    Ok(options)
}
