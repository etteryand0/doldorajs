use std::fmt;

#[allow(non_camel_case_types)]
#[derive(fmt::Debug)]
pub enum ErrorCode {
    ERR_NOT_ENOUGH_ARGUMENTS,
    ERR_INVALID_PROJECT_NAME,
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