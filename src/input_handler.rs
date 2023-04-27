use std::io;
use std::io::Write;
use std::path::{MAIN_SEPARATOR, MAIN_SEPARATOR_STR};
use regex::Regex;
use lazy_static::lazy_static;

/// Prompts the user to input the path to their working directory.
/// Verifies correct formatting using a regular expression.
pub fn get_working_dir() -> Result<String, io::Error> {
    let mut working_dir: String = String::new();

    lazy_static! {
        // Only allowing alphanumeric characters and slashes.
        static ref PROJECT_REGEX: Regex = Regex::new(r"^([\p{L}\p{N}/\\]+)$").unwrap();
    }

    while !PROJECT_REGEX.is_match(&working_dir) {
        print!("Please enter the path to your working directory: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut working_dir).unwrap();
        working_dir = working_dir.trim().parse().unwrap();

        // Ensure path slashes are correct for the environment.
        working_dir = working_dir.replace("/", MAIN_SEPARATOR_STR);
        working_dir = working_dir.replace("\\", MAIN_SEPARATOR_STR);

        // Strip trailing slashes.
        while working_dir.ends_with(MAIN_SEPARATOR_STR) {
            working_dir.pop();
        }
        if !working_dir.starts_with(MAIN_SEPARATOR) {
            working_dir = format!("{}{}", MAIN_SEPARATOR, working_dir);
        }

        if !PROJECT_REGEX.is_match(&working_dir) {
            println!("Working directory is invalid.");
            working_dir.clear();
        }
    }
    Ok(working_dir)
}

/// Prompts the user to input their project name.
/// Verifies correct formatting using a regular expression.
pub fn get_project_name() -> Result<String, io::Error> {
    let mut project_name: String = String::new();
    lazy_static! {
        // Only allowing alphanumeric characters.
        static ref PROJECT_REGEX: Regex = Regex::new(r"^([\p{L}\p{N}]+)$").unwrap();
    }

    while !PROJECT_REGEX.is_match(&project_name) {
        print!("Please enter your project name: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut project_name).unwrap();
        project_name = project_name.trim_end().parse().unwrap();

        if !PROJECT_REGEX.is_match(&project_name) {
            println!("Project name is invalid.");
            project_name.clear();
        }
    }
    Ok(project_name)
}

/// Prompts the user for input to receive confirmation to perform an action.
pub fn user_confirm() -> Result<(), ()> {
    let mut confirm: String = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut confirm).unwrap();
    confirm = confirm.trim_end().to_lowercase();

    return match confirm.as_str() {
        "y" => Ok(()),
        _ => Err(())
    }
}