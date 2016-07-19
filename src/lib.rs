//! SBrain, or Semantic Brain, is a set of extensions to the famous language by Urban Müller
//! designed to make it more amenable to genetic programming. Additions include a stack, a general-
//! purpose register, and single-instruction arithmetic.
//!
//! This crate provides an implementation of the SBrain specification designed to be used for
//! genetic programming. See the `specification` pseudomodule for the complete specification.
//!
//! Here's a quick example:
//!
//! ```
//! # use sbrain::*;
//! let result = evaluate("[.>]@@Test Data to Echo", None);
//! assert_eq!("Test Data to Echo", &tape_to_string(result.output));
//! // This program terminates after 52 cycles
//! assert_eq!(52, result.cycles);
//!
//! // In this case, the program is interruped before completion.
//! let result = evaluate("[.>]@@Test Data to Echo", Some(32));
//! // The program doesn't finish, because it would take more than 32 cycles.
//! assert_eq!("Test Data t", &tape_to_string(result.output));
//! assert_eq!(false, result.halted);
//! ```

pub mod specification;
mod machine;
mod source;

pub use machine::*;
pub use source::source_to_tapes;

/// Represents the outcome of an evaluation by the SBrain VM.
pub struct EvalResult {
    /// The output of the computation
    pub output: Vec<MData>,
    /// The number of cycles for which the machine ran
    pub cycles: u32,
    /// Whether or not the machine halted on its own. False means it was interrupted.
    pub halted: bool,
}

/// Run the program represented by the given source on a new Semantic Brain VM.
/// If Limit is None, this may never return; if it is Some(n), the machine will run for at most n
/// cycles, then stop.
///
/// # Panics
/// This function panics if the source evaluates to tapes that exceed the maximum size of the
/// VM's tapes (2^16 )
///
/// # Examples
/// This simple program reads data off the tape until it encounters a 0. Here, `evaluate()` is used
/// without an execution limit; this is because it's easy to reason about when the program will
/// end. This isn't recommended for any but the simplest programs.
///
/// ```
/// # use sbrain::*;
/// let result = evaluate("[.>]@@Test Data to Echo", None);
///
/// assert_eq!("Test Data to Echo", &tape_to_string(result.output));
/// // This program terminates after 52 cycles
/// assert_eq!(52, result.cycles);
/// ```
/// In this case, the program is interruped before completion.
///
/// ```
/// # use sbrain::*;
/// let result = evaluate("[.>]@@Test Data to Echo", Some(32));
/// // The program doesn't finish, because it would take more than 32 cycles.
/// assert_eq!("Test Data t", &tape_to_string(result.output));
/// assert_eq!(false, result.halted);
/// ```
pub fn evaluate(source: &str, limit: Option<u32>) -> EvalResult {
    // Transliterate the source code, creating Vec<MData> tapes.
    let (program, data) = source_to_tapes(&source);
    // Create a machine with no input tape.
    let mut machine = SBrainVM::new(None);
    // Load the program and data tapes.
    machine.load_program(&program).unwrap();
    machine.load_data(&data).unwrap();

    let (cycles, halted) = machine.run(limit);

    EvalResult {
        output: machine.get_output(),
        cycles: cycles,
        halted: halted,
    }
}

/// Functions much like `evaluate()`, but provides the VM with a fixed input tape.
///
/// ```
/// # use sbrain::*;
/// let result = fixed_evaluate(",.,.,.,.,.@", Some(vec![72, 101, 108, 108, 111]), None);
///
/// assert_eq!("Hello", &tape_to_string(result.output));
/// ```
pub fn fixed_evaluate(source: &str, input: Option<Vec<MData>>, limit: Option<u32>) -> EvalResult {
    // Transliterate the source code, creating Vec<MData> tapes.
    let (program, data) = source_to_tapes(&source);
    // Create a machine
    let mut machine: SBrainVM;
    if let Some(v) = input {
        machine = SBrainVM::new(Some(v.iter().rev().cloned().collect()));
    } else {
        machine = SBrainVM::new(None);
    }
    // Load the program and data tapes.
    machine.load_program(&program).unwrap();
    machine.load_data(&data).unwrap();

    let (cycles, halted) = machine.run(limit);

    EvalResult {
        output: machine.get_output(),
        cycles: cycles,
        halted: halted,
    }

}



/// Convert a tape of MData cells into Unicode chars. Invalid chars are excluded, which could have
/// some unintended side effects for genesis based on string comparisons.
pub fn tape_to_string(tape: Vec<MData>) -> String {
    use std::char;
    let mut result = String::with_capacity(tape.len());
    for cell in tape {
        if let Some(c) = char::from_u32(cell) {
            result.push(c);
        };
    }
    result
}
