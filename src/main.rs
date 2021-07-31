mod error;

use error::{DError, ErrorCode};

use std::env;

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
                status: ErrorCode::ERR_NOT_ENOUGH_ARGUMENTS,
            })
        }
        _ => (),
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

    match status {
        Ok(_) => (),
        Err(e) => {
            println!("\x1b[31m{:?}\x1b[0m: {}", e.status, e.message);
            std::process::exit(1);
        }
    }

    // actual doldora funcs ...
}
