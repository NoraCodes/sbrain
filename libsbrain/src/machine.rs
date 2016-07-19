//! SBrain VM data structure definitions
use std::io::Read;
use std::io;

/// The type of a data cell
pub type MData = u32;
/// The type of a pointer to a cell.
pub type MAddr = u16;

/// A virtual machine modelling the SBrain Turing machine.
/// This machine implements the specification relatively strictly, providing exactly 2^16 (65536) data and
/// instruction cells. Thus, all pointers are u16. All data is u32.
/// The main deviation from the minimum specification is the jump stack, which is indefinitely
/// expandable.

pub struct SBrainVM {
    // Data containers
    /// The data tape contains the primary data on which the program will operate
    /// 16-bit addresses with a single dead address
    data_tape: [MData; 65536],
    /// The data stack allows the position-independent storage of data
    data_stack: Vec<MData>,
    /// Auxiliary register (auxi_r)
    auxi_r: MData,

    // Machine Internals
    /// The jump stack contains addresses on the data tape; 16 bit values are all that are
    /// necessary.
    jump_stack: Vec<MAddr>,
    /// The instruction tape contains instructions. This VM uses the recommended 6-bit binary
    /// format, but Rust does not have a 6-bit datatype, so u8 is used instead
    exec_tape: [u8; 65536],
    /// Pointer to the current data cell
    data_p: MAddr,
    /// Pointer to the current instruction
    inst_p: MAddr,
    /// Pointer to the next jump position
    jump_p: MAddr,

    // I/O Tapes
    input_t: Option<Vec<MData>>,
    output_t: Vec<MData>,
}

/// FlowAction allows the VM's execution engine to implement flow control
/// Because evaluation can only see a single instruction, it must use this struct to instruct the flow
/// controller to perform flow control actions.
pub enum FlowAction {
    /// No flow control action is required
    NoAction,
    /// The flow controller should skip to the next 5 (`]`), or loop end instructon
    SkipLoop,
    /// The program is done.
    Done,
}

impl SBrainVM {
    /// Return a new SBrainVM, with no data in any tapes.
    pub fn new(input_t: Option<Vec<MData>>) -> SBrainVM {
        SBrainVM {
            data_tape: [0; 65536],
            data_stack: vec![0; 256],
            auxi_r: 0,
            jump_stack: Vec::new(),
            exec_tape: [0; 65536],
            data_p: 0,
            inst_p: 0,
            jump_p: 0,

            input_t: input_t,
            output_t: Vec::new(),
        }
    }

    /// Return a new SBrainVM in a Box<>, with no data in any tapes.
    pub fn boxed(input_t: Option<Vec<MData>>) -> Box<SBrainVM> {
        Box::new(SBrainVM::new(input_t))
    }

    /// Load a program tape: copy data from the given slice into the executable tape.
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

    /// Load a data tape: copy data from the given slice into the VM's data tape.
    /// On error, the Err(s) return will contain a message describing the error.
    pub fn load_data(&mut self, data: &[MData]) -> Result<(), String> {
        // No data can be longer than the data tape
        if data.len() > 65536 {
            return Err(String::from("Provided data exceeds VM tape length."));
        }

        // Target is a slice of the VMs data tape, of the same size as the incoming data
        // This is required for clone_from_slice
        self.data_tape[0..data.len()].clone_from_slice(data);
        return Ok(());
    }

    fn get_input(&mut self) -> MData {
        match &mut self.input_t {
            &mut Some(ref mut v) => {
                match v.pop() {
                    Some(n) => n,
                    None => 0,
                }
            }
            &mut None => {
                // No tape; get a byte from stdin
                io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u32)
                    .unwrap_or(0)
            }
        }
    }

    fn put_output(&mut self, output: MData) {
        self.output_t.push(output);
    }

    /// Execute an instruction on the current virtual machine
    fn do_instruction(&mut self, instr: u8) -> FlowAction {
        match instr {
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
                self.data_tape[self.data_p as usize] = self.data_tape[self.data_p as usize]
                    .wrapping_sub(1);
            }
            3 => {
                self.data_tape[self.data_p as usize] = self.data_tape[self.data_p as usize]
                    .wrapping_add(1);
            }
            // Jump instructions
            4 => {
                self.jump_p = self.inst_p;
                self.jump_stack.push(self.jump_p);
                // If *data_p is 0, the flow controller needs to skip to the next 5
                if self.data_tape[self.data_p as usize] == 0 {
                    return FlowAction::SkipLoop;
                }
            }
            5 => {
                self.jump_p = match self.jump_stack.pop() {
                    Some(n) => n,
                    None => 0,
                };
                // If *data_p isn't 0, jump to the instruction just retrieved
                if self.data_tape[self.data_p as usize] != 0 {
                    self.inst_p = self.jump_p;
                }
            }
            // I/O commands
            6 => {
                let temp = self.data_tape[self.data_p as usize];
                self.put_output(temp);
            }
            7 => {
                let temp = self.get_input();
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
            //  Left Shift
            14 => {
                self.auxi_r = self.auxi_r << 1;
            }
            //  Right Shift
            15 => {
                self.auxi_r = self.auxi_r >> 1;
            }

            // Aux/tape operations
            //  OR
            16 => {
                self.data_tape[self.data_p as usize] = self.data_tape[self.data_p as usize] |
                                                       self.auxi_r;
                println!("or: {:?}", self.data_tape[self.data_p as usize]);
            }
            //  AND
            17 => {
                self.data_tape[self.data_p as usize] = self.data_tape[self.data_p as usize] &
                                                       self.auxi_r;
            }
            //  XOR
            18 => {
                self.data_tape[self.data_p as usize] = self.data_tape[self.data_p as usize] ^
                                                       self.auxi_r;
            }
            //  NOR
            19 => {
                self.data_tape[self.data_p as usize] = !(self.data_tape[self.data_p as usize] |
                                                         self.auxi_r);
            }
            //  NAND
            20 => {
                self.data_tape[self.data_p as usize] = !(self.data_tape[self.data_p as usize] &
                                                         self.auxi_r);
            }
            //  ADD
            21 => {
                self.data_tape[self.data_p as usize] = self.data_tape[self.data_p as usize]
                    .wrapping_add(self.auxi_r);
            }
            //  DIFFERENCE
            22 => {
                self.data_tape[self.data_p as usize] = self.data_tape[self.data_p as usize]
                    .wrapping_sub(self.auxi_r);
            }
            //  QUOTIENT
            23 => {
                self.data_tape[self.data_p as usize] = self.data_tape[self.data_p as usize]
                    .wrapping_div(self.auxi_r);
            }
            //  MODULO
            24 => {
                self.data_tape[self.data_p as usize] = self.data_tape[self.data_p as usize] %
                                                       self.auxi_r;
            }
            //  PRODUCT
            25 => {
                self.data_tape[self.data_p as usize] = self.data_tape[self.data_p as usize]
                    .wrapping_mul(self.auxi_r);
            }
            31 => {
                return FlowAction::Done;
            }
            _ => {}
        }
        return FlowAction::NoAction;
    }

    /// Return the address of the next occurence of a given instruction
    fn find_next(&self, target_instr: u8) -> MAddr {
        // Look only after inst_p
        for (addr, instr) in (&self.exec_tape[self.inst_p as usize..]).iter().enumerate() {
            // Once found, return
            if *instr == target_instr {
                return (addr - 1) as MAddr;
            };
        }
        // If not found, return the end of the tape. This allows broken programs to exit early,
        // typically.
        return (&self.exec_tape.len() - 1) as MAddr;
    }

    /// Return a copy of the output data of the machine
    pub fn get_output(&self) -> Vec<MData> {
        return self.output_t.clone();
    }


    /// Run the machine, until completion (cycles = None) or for n cycles (cycles = Some(n)).
    /// Return values are number of cycles run and why the machine stopped: false if due to a
    /// program halt (instr 31), false if due to running out of cycles.
    pub fn run(&mut self, cycles: Option<u32>) -> (u32, bool) {
        let mut done_cycles = 0;

        // The main execution loop
        loop {
            // Execute the current instruction.
            let instruction = self.exec_tape[self.inst_p as usize].clone();
            let action = self.do_instruction(instruction);

            // Take the appropriate action based on action
            match action {
                // Advance the tape
                FlowAction::NoAction => self.inst_p += 1,
                // Quit
                FlowAction::Done => return (done_cycles, true),
                // Skip to the end of a loop
                FlowAction::SkipLoop => {
                    self.inst_p = self.find_next(5);
                }
            }
            // Increment the cycle count
            done_cycles += 1;
            if let Some(n) = cycles {
                if done_cycles >= n {
                    return (done_cycles, false);
                }
            }
        }
    }
}
