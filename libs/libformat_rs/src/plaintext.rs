//! Plain text file loading.

use crate::FormatError;
use std::path::Path;

pub fn load_plaintext(path: &Path) -> Result<String, FormatError> {
    Ok(std::fs::read_to_string(path)?)
}
