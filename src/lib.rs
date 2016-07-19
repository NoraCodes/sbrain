//! SBrain, or Semantic Brain, is a set of extensions to the famous language by Urban Müller
//! designed to make it more amenable to genetic programming. Additions include a stack, a general-
//! purpose register, and single-instruction arithmetic.
//!
//! This crate provides an implementation of the SBrain specification designed to be used for
//! genetic programming. See the `specification` pseudomodule for the complete specification.


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
