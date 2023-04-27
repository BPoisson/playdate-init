use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Lines};
use std::path::MAIN_SEPARATOR;
use std::process::{Child, ChildStdout, Stdio};
use crate::input_handler::user_confirm;

/// Runs 'cmake ..' to initialize the project for Visual Studio.
pub fn run_cmake(project_path: &String) -> Result<(), io::Error> {
    print!("Initialize project with CMake? [Y/N]: ");

    match user_confirm() {
        Ok(_) => {
            println!("Running CMake...");

            // Spawn child to run command and capture output.
            let mut cmake_output: Child = std::process::Command::new("cmd")
                .args(&["/C", "cmake .."])
                .current_dir(format!("{}{}build", project_path, MAIN_SEPARATOR))
                .stdout(Stdio::piped())
                .spawn()
                .expect("CMake initialization failed.");

            let stdout: ChildStdout = cmake_output.stdout.take().unwrap();

            // Stream output.
            let lines: Lines<BufReader<ChildStdout>> = BufReader::new(stdout).lines();
            for line in lines {
                println!("{}", line.unwrap());
            }
            println!("CMake successful!");
            Ok(())
        },
        Err(_) => Err(io::Error::new(ErrorKind::Other, "Skipping CMake initialization."))
    }
}