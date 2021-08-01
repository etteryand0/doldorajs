use std::fmt;

#[allow(non_camel_case_types)]
#[derive(fmt::Debug)]
pub enum ErrorCode {
    ERR_NOT_ENOUGH_ARGUMENTS,
    ERR_INVALID_PROJECT_NAME,
    ERR_IO_STDIN_FAILURE,
    ERR_FS_FAILURE,
}

#[derive(fmt::Debug)]
pub struct DError {
    pub message: String,
    pub code: ErrorCode,
}

// impl for DError to println! ErrorCode
impl fmt::Display for DError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn throw_error(e: DError) {
    println!("\x1b[31m{:?}\x1b[0m: {}", e.code, e.message);
    std::process::exit(1);
}
