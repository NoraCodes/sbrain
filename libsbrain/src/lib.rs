#[allow(dead_code)]
pub mod machine;
pub use machine::{MData, MAddr};
#[allow(dead_code)]
pub mod source;

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
/// If Limit is Mone, this may never return; if it is Some(n), the machine will run for only n
/// cycles, then stop.
pub fn evaluate(source: &str, limit: Option<u32>) -> EvalResult {
    // Transliterate the source code, creating Vec<MData> tapes.
    let (program, data) = source::source_to_tapes(&source);
    // Create a machine with no input tape.
    let mut machine = machine::SBrainVM::new(None);
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
