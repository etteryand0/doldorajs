use std::fs;
use crate::{
  DError,
  ErrorCode,
};

pub fn create_dir(path: &String) -> Result<(), DError> {
  match fs::create_dir(path) {
    Ok(_) => Ok(()),
    Err(e) => Err(DError {
      message: e.to_string(),
      code: ErrorCode::ERR_FS_FAILURE,
    }),
  }
}
