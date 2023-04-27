use std::{fs, io};
use std::io::ErrorKind;
use std::path::{MAIN_SEPARATOR, Path};
use crate::input_handler::user_confirm;

/// Creates the 'src' and 'build' directories within the project directory.
/// Creates a 'CMakeLists.txt' file in the project root directory from a template,
///     with the user's project name included in the file where necessary.
/// Creates a 'main.c' file in the project's 'src' directory by creating a copy from a template.
pub fn create_project_contents(project_path: &String, project_name: &String) -> Result<(), io::Error> {
    println!("Creating project workspace...");

    fs::create_dir(format!("{}{}{}", project_path, MAIN_SEPARATOR, "src"))?;
    fs::create_dir(format!("{}{}{}", project_path, MAIN_SEPARATOR, "build"))?;

    let make_file_contents = fs::read_to_string(
        format!(".{}templates{}CMakeListsTemplate.txt", MAIN_SEPARATOR, MAIN_SEPARATOR))
        .unwrap()
        .replace("PROJECT_NAME_PLACEHOLDER", project_name);

    fs::write(format!("{}{}CMakeLists.txt", project_path, MAIN_SEPARATOR), make_file_contents)
        .unwrap();

    fs::copy(format!(".{}templates{}mainTemplate.c", MAIN_SEPARATOR, MAIN_SEPARATOR),
             format!("{}{}src{}main.c", project_path, MAIN_SEPARATOR, MAIN_SEPARATOR))
        .unwrap();

    println!("Project workspace creation successful!");

    Ok(())
}

/// Creates the project directory by concatenating the user's project name with their working directory.
pub fn create_project_dir(project_name: &String, project_dir: &String) -> Result<String, io::Error> {
    let project_path = format!("{}{}{}", project_dir, MAIN_SEPARATOR, project_name);

    print!("Create project at {}? [Y/N]: ", project_path);

    match user_confirm() {
        Ok(_) => {
            if Path::new(&project_path).exists() {
                return Err(io::Error::new(ErrorKind::Other, "Project directory already exists."))
            }

            fs::create_dir_all(&project_path)?;

            Ok(project_path)
        },
        Err(_) => Err(io::Error::new(ErrorKind::Other, "User Declined."))
    }
}