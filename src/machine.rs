//! The implementation of the SBrain VM.
use crate::{MAddr, MData};
use std::io;
use std::io::{Read, Write};
use std::u16::MAX as u16MAX;

/// A virtual machine modelling the SBrain Turing machine.
/// This machine implements the specification relatively strictly, providing exactly 2^16 (65536)
/// data and instruction cells. Thus, all pointers are 16 bits and all data is 8 bits.
/// The main deviation from the minimum specification is the jump stack, which is indefinitely
/// expandable.

pub struct SBrainVM<'a> {
    // Data containers
    /// The data tape contains the primary data on which the program will operate
    /// 16-bit addresses with a single dead address
    data_tape: [MData; 65536],
    /// The data stack allows the position-independent storage of data
    data_stack: Vec<MData>,
    /// Auxiliary register (auxi_r)
    auxi_r: MData,

    // Machine Internals
    /// The instruction tape contains instructions. This VM uses the recommended 6-bit binary
    /// format, but Rust does not have a 6-bit datatype, so u8 is used instead
    exec_tape: [u8; 65536],
    /// Pointer to the current data cell
    data_p: MAddr,
    /// Pointer to the current instruction
    inst_p: MAddr,

    // I/O Tapes
    input_t: Option<&'a mut dyn Read>,
    output_t: Option<&'a mut dyn Write>,
}

impl<'a> SBrainVM<'a> {
    /// Return a new SBrainVM, with no data in any tapes.
    /// If given a `None` `input`, all reads read 0.
    /// If given a `None` `output`, all writes are discarded.
    pub fn new(
        input: Option<&'a mut dyn Read>,
        output: Option<&'a mut dyn Write>,
        program: &[u8],
    ) -> Result<SBrainVM<'a>, String> {
        let mut new = SBrainVM {
            data_tape: [0; 65536],
            data_stack: vec![0; 256],
            auxi_r: 0,
            exec_tape: [0; 65536],
            data_p: 0,
            inst_p: 0,

            input_t: input,
            output_t: output,
        };
        new.load_program(program)?;
        Ok(new)
    }

    /// Load a program tape: copy data from the given slice into the executable tape,
    /// starting at address zero.
    /// On error, the Err(s) return will contain a message describing the error.
    pub fn load_program(&mut self, program: &[u8]) -> Result<(), String> {
        // No program can be longer than the tape the VM stores programs on.
        if program.len() > 65536 {
            return Err(String::from("Provided program exceeds VM tape length."));
        }

        // Target is a slice of the VMs executable tape of the same size as the program
        // This is required from clone_from_slice
        self.exec_tape[0..program.len()].clone_from_slice(program);
        return Ok(());
    }

    fn get_input(&mut self) -> io::Result<MData> {
        let mut buf = [0; 1];
        if let Some(ref mut r) = self.input_t {
            r.read(&mut buf)?;
            Ok(buf[0])
        } else {
            Ok(0)
        }
    }

    fn put_output(&mut self, output: MData) -> io::Result<()> {
        match &mut self.output_t {
            &mut Some(ref mut w) => {
                w.write(&[output])?;
                Ok(())
            }
            &mut None => Ok(()),
        }
    }

    /// Execute an instruction on the current virtual machine
    /// Returns true if execution is finished and false if not
    fn do_instruction(&mut self) -> io::Result<bool> {
        match self.exec_tape[self.inst_p as usize] {
            // wrapping_add() and wrapping_sub are used in order to never overflow the bounds
            // of unsigned int types
            //
            // Decr. and incr. for data_p
            0 => {
                self.data_p = self.data_p.wrapping_sub(1);
            }
            1 => {
                self.data_p = self.data_p.wrapping_add(1);
            }
            // Decr. and incr. for *data_p
            2 => {
                self.data_tape[self.data_p as usize] =
                    self.data_tape[self.data_p as usize].wrapping_sub(1);
            }
            3 => {
                self.data_tape[self.data_p as usize] =
                    self.data_tape[self.data_p as usize].wrapping_add(1);
            }
            // Jump instructions
            4 => {
                // If *data_p is 0, skip forward to the corresponding 5
                let this_inst = self.inst_p;
                if self.data_tape[self.data_p as usize] == 0 {
                    let mut nest_level = 1;
                    while nest_level > 0 {
                        self.inst_p = self.inst_p.wrapping_add(1);
                        if self.inst_p == 0 {
                            self.inst_p = this_inst;
                            break;
                        }
                        if self.exec_tape[self.inst_p as usize] == 4 {
                            nest_level += 1;
                        } else if self.exec_tape[self.inst_p as usize] == 5 {
                            nest_level -= 1;
                        }
                    }
                }
            }
            5 => {
                // If *data_p isn't 0, skip backward to the corresponding 4
                let this_inst = self.inst_p;
                if self.data_tape[self.data_p as usize] != 0 {
                    let mut nest_level = 1;
                    while nest_level > 0 {
                        self.inst_p = self.inst_p.wrapping_sub(1);
                        if self.inst_p == u16MAX {
                            self.inst_p = this_inst;
                            break;
                        }
                        if self.exec_tape[self.inst_p as usize] == 5 {
                            nest_level += 1;
                        } else if self.exec_tape[self.inst_p as usize] == 4 {
                            nest_level -= 1;
                        }
                    }
                }
            }
            // I/O commands
            6 => {
                let temp = self.data_tape[self.data_p as usize];
                self.put_output(temp)?;
            }
            7 => {
                let temp = self.get_input()?;
                self.data_tape[self.data_p as usize] = temp;
            }
            // Stack instructions
            8 => {
                self.data_stack.push(self.data_tape[self.data_p as usize]);
            }
            9 => {
                self.data_tape[self.data_p as usize] = match self.data_stack.pop() {
                    Some(n) => n,
                    None => 0,
                };
            }
            // Aux register instructions
            10 => {
                self.auxi_r = self.data_tape[self.data_p as usize];
            }
            11 => {
                self.data_tape[self.data_p as usize] = self.auxi_r;
            }
            12 => {
                self.auxi_r = 0;
            }
            // Bitwise auxi_r instructions
            //  NOT
            13 => self.auxi_r = !self.auxi_r,
            //  AND
            14 => {
                self.auxi_r = self.data_tape[self.data_p as usize] & self.auxi_r;
            }
            15 => {
                return Ok(true);
            }
            _ => {}
        }
        return Ok(false);
    }

    fn nexti(&mut self) -> bool {
        // increment the PC
        self.inst_p = self.inst_p.wrapping_add(1);
        // if it went over, wrap it and inform the caller
        if self.inst_p as usize == self.exec_tape.len() - 1 {
            self.inst_p = 0;
            return true;
        }
        return false;
    }

    /// Run the machine, until completion (cycles = None) or for n cycles (cycles = Some(n)).
    /// Return values are number of cycles run and the return code, or None if the code simply ran
    /// out of cycles.
    pub fn run(&mut self, cycles: Option<u32>) -> io::Result<(u32, Option<u8>)> {
        let mut done_cycles = 0;

        // The main execution loop
        loop {
            // Execute the current instruction.
            if self.do_instruction()? {
                return Ok((done_cycles, Some(self.auxi_r)));
            } else {
                self.nexti();
            }

            // Increment the cycle count
            done_cycles += 1;
            if let Some(n) = cycles {
                if done_cycles >= n {
                    return Ok((done_cycles, None));
                }
            }
        }
    }
}
