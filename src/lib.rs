//! SBrain, or Semantic Brain, is a set of extensions to the famous language by Urban Müller
//! designed to make it more amenable to genetic programming. Additions include a stack, a general-
//! purpose register, and useful bitwise operations.
//!
//! This crate provides an implementation of the SBrain specification designed to be used for
//! genetic programming. See the `specification` pseudomodule for the complete specification.
//!
//! Here's a quick example:
//!
//! ```
//! # use sbrain::*;
//! let program = source_to_tape(",[.,]");
//! let mut input = make_input_vec(b"Hello, world!");
//! let mut output = make_output_vec();
//! SBrainVM::new(Some(&mut input), Some(&mut output), &program)
//!     .expect("Could not build machine")
//!     .run(Some(1000)).expect("I/O failed");
//!
//! let output = output.into_inner();
//! assert_eq!(&output, b"Hello, world!")
//! ```

mod machine;
mod source;
pub mod specification;
mod tapes;

pub use machine::*;
pub use source::source_to_tape;
pub use tapes::{make_input_vec, make_output_vec, tape_to_string};

use std::io;

/// The type of a data cell
pub type MData = u8;
/// The type of a pointer to a cell.
pub type MAddr = u16;

/// Converts the given source code to a SBrain executable and runs it, taking input from stdin and doing output on stdout.
///
/// # Panics
/// Panics if there is an I/O error with standard in or standard out.
pub fn simple_run(source: &str) -> u8 {
    let program = source_to_tape(source);
    SBrainVM::new(Some(&mut io::stdin()), Some(&mut io::stdout()), &program)
        .expect("Could not build machine")
        .run(None)
        .expect("Unable to run program")
        .1
        .expect("Program did not terminate")
}
