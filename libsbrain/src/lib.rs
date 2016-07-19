
#[allow(dead_code)]
pub mod specification;
mod machine;
/// These datatypes are used to represent the data and address cells and registers in the machine.
pub use machine::*;
#[allow(dead_code)]
mod source;
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
