mod error;
mod tests;
mod command_line;

use error::{DError, ErrorCode, throw_error};
use command_line::{
    create_dir,
};

use regex::Regex;
use std::{
    env, 
    io::{self, Write}
};

// \x1b[33m - Yellow
// \x1b[32m - Green
// \x1b[31m - Red
// \x1b[0m - Reset
const USAGE_TEXT: &str = "\x1b[33mUsage of Doldora\x1b[0m: doldora <project_name> \
                    \n    there is no optional flags for now\n";

// TODO. fn to validate arguments and proccess/analyze them (in future)
fn check_arguments() -> Result<(), DError> {
    // get command line arguments and it's count
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    let mut error: Option<DError> = None;

    // if command line arguments count is 0 or 1, then add error
    match argc {
        0 | 1 => {
            error = Some(DError {
                message: String::from("You have to provide at least a project name \n")
                    + USAGE_TEXT,
                code: ErrorCode::ERR_NOT_ENOUGH_ARGUMENTS,
            })
        }
        _ => {
            // first char can only be [a-z]
            // other chars can only be [a-z], _, +, 0-9
            let expression = r"^([^a-z])|[^a-z_0-9-]+?";
            let project_name_re = Regex::new(expression).unwrap();
            
            if project_name_re.is_match(&argv[1]) {
                error = Some(DError {
                    message: String::from(expression) 
                        + " Project name can only begin with [a-z] and can't contain [A-Z] or any special character except _-",
                    code: ErrorCode::ERR_INVALID_PROJECT_NAME
                })
            }
        }
    }

    // if error is initialized, then return Err()
    if let Some(_) = error {
        return Err(error.unwrap());
    }

    Ok(())
}

// print doldora version and authors
fn attributition() {
    let authors = env!("CARGO_PKG_AUTHORS").split(":");
    println!(
        "\x1b[32m{} {} by",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    for author in authors {
        println!("\x1b[32m  {}\x1b[0m", author);
    }
}

fn main() {
    attributition();
    let status = check_arguments();
    let argv: Vec<String> = env::args().collect();

    match status {
        Ok(_) => {
            print!("Creating project named \x1b[36m'{}'\x1b[0m, proceed? [Y,n]: ", &argv[1]);
            io::stdout().flush().unwrap();
            
            let mut confirmation = String::new();
            match io::stdin().read_line(&mut confirmation) {
                Ok(_) => (),
                Err(_) => throw_error(DError {
                    message: String::from("Error while reading input"),
                    code: ErrorCode::ERR_IO_STDIN_FAILURE,
                })
            }

            confirmation = String::from(confirmation.to_lowercase().trim());
            if confirmation == "n" 
            || confirmation != "y" {
                println!("Aborting... ");
                std::process::exit(0);
            }
        },
        Err(e) => throw_error(e),
    }

    match create_dir(&argv[1]) {
        Ok(_) => (),
        Err(e) => throw_error(e),
    }
}
