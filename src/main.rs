mod input_handler;
mod resource_creator;
mod initializer;

use std::io;
use crate::input_handler::{get_working_dir, get_project_name};
use crate::resource_creator::{create_project_contents, create_project_dir};
use crate::initializer::{run_cmake};

fn main() -> Result<(), io::Error>{
    println!("Welcome to the Playdate Visual Studio CMake project initializer!\n");

    let project_name: String = get_project_name()
        .expect("Error reading project name.");
    let project_dir: String = get_working_dir()
        .expect("Error reading project directory.");

    let project_dir_result: String = match create_project_dir(&project_name, &project_dir) {
        Ok(result) => result,
        Err(msg) => {
            println!("{}", msg);
            println!("Aborting...");
            return Ok(());
        }
    };

    create_project_contents(&project_dir_result, &project_name)?;

    match run_cmake(&project_dir_result) {
        Ok(()) => {
            println!("\nProject initialization complete!");
        },
        Err(msg) => {
            println!("{}", msg);
            println!("\nProject creation complete without CMake!");
        }
    }
    return Ok(());
}


