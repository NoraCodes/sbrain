use crate::MData;
use std::borrow::Cow;
use std::io::Cursor;

/// Convert a tape of MData cells into Unicode chars. Invalid chars are excluded, which could have
/// some unintended side effects for genesis based on string comparisons.
pub fn tape_to_string<'a>(tape: &'a [MData]) -> Cow<'a, str> {
    String::from_utf8_lossy(&tape)
}

/// Create a new Cursor-wrapped input vector which can be used by a machine to read from.
pub fn make_input_vec(data: &[u8]) -> Box<Cursor<Vec<u8>>> {
    Box::new(Cursor::new(data.to_vec()))
}

/// Create a new Cursor-wrapped output vector which can be used by a machine to write onto.
pub fn make_output_vec() -> Box<Cursor<Vec<u8>>> {
    Box::new(Cursor::new(Vec::new()))
}
