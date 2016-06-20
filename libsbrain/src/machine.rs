//! SBrain VM data structure definitions

/// A virtual machine modelling the SBrain Turing machine.
/// This machine implements the specification relatively strictly, providing exactly 2^16 (65536) data and
/// instruction cells. Thus, all pointers are u16. All data is u32.
/// The main deviation from the minimum specification is the jump stack, which is indefinitely
/// expandable.
pub struct SBrainVM {
    // Data containers
    /// The data tape contains the primary data on which the program will operate
    /// 16-bit addresses with a single dead address
    data_tape: [u32; 65536], 
    /// The data stack allows the position-independent storage of data
    data_stack: Vec<u32>,
    /// Auxiliary register (auxi_r)
    auxi_r: u32,

    // Machine Internals
    /// The jump stack contains addresses on the data tape; 16 bit values are all that are
    /// necessary.
    jump_stack: Vec<u16>,
    /// The instruction tape contains instructions. This VM uses the recommended 6-bit binary
    /// format, but Rust does not have a 6-bit datatype, so u8 is used instead
    exec_tape: [u8; 65536],
    /// Pointer to the current data cell
    data_p: u16,
    /// Pointer to the current instruction
    inst_p: u16,
    /// Pointer to the next jump position
    jump_p: u16,
    
    // I/O Tapes
    input_t: Vec<u32>,
    output_t: Vec<u32>
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
    Done
}

impl SBrainVM {
    /// Return a new SBrainVM, with no data in any tapes.
    pub fn new(input_t: Vec<u32>) -> SBrainVM {
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
    pub fn boxed(input_t: Vec<u32>) -> Box<SBrainVM> {
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
    pub fn load_data(&mut self, data: &[u32]) -> Result<(), String> {
        // No data can be longer than the data tape
        if data.len() > 65536 {
            return Err(String::from("Provided data exceeds VM tape length."));
        }

        // Target is a slice of the VMs data tape, of the same size as the incoming data
        // This is required for clone_from_slice
        self.data_tape[0..data.len()].clone_from_slice(data);
        return Ok(());
    }

    fn get_input(&mut self) -> u32 {
        match self.input_t.pop() {
            Some(n) => n,
            None => 0
        }
    }

    fn put_output(&mut self, output: u32) {
        self.output_t.push(output);
    }

    /// Execute an instruction on the current virtual machine
    fn do_instruction(&mut self, instr: u8) -> FlowAction {
        match instr {
            // wrapping_add() and wrapping_sub are used in order to never overflow the bounds
            // of unsigned int types
            
            // Decr. and incr. for data_p
            0 => { self.data_p.wrapping_sub(1); },
            1 => { self.data_p.wrapping_add(1); },
            // Decr. and incr. for *data_p
            2 => { self.data_tape[self.data_p as usize].wrapping_sub(1); },
            3 => { self.data_tape[self.data_p as usize].wrapping_add(1); },
            // Jump instructions
            4 => { self.jump_p = self.inst_p;
                   self.jump_stack.push(self.jump_p);
                   // If *data_p is 0, the flow controller needs to skip to the next 5
                   if self.data_tape[self.data_p as usize] == 0 {
                    return FlowAction::SkipLoop; 
                   } 
                 },
            5 => { self.jump_p = match self.jump_stack.pop(){
                                    Some(n) => n,
                                    None => 0
                                 };
                   // If *data_p isn't 0, jump to the instruction just retrieved
                   if self.data_tape[self.data_p as usize] != 0 {
                       self.inst_p = self.jump_p;
                   }
                 },
            // I/O commands
            6 => { 
                let temp = self.data_tape[self.data_p as usize];
                self.put_output(temp); },
            7 => { 
                let temp = self.get_input();
                self.data_tape[self.data_p as usize] = temp; },
            // Stack instructions
            8 => { self.data_stack.push(
                    self.data_tape[self.data_p as usize]); }
            9 => { self.data_tape[self.data_p as usize] = match self.data_stack.pop() {
                                                    Some(n) => n,
                                                    None => 0,
                                                 };
                 },
            // Aux register instructions
            10 => { self.auxi_r = self.data_tape[self.data_p as usize]; },
            11 => { self.data_tape[self.data_p as usize] = self.auxi_r; },
            12 => { self.auxi_r = 0; },
            // Bitwise auxi_r instructions
            //  NOT
            13 => { self.auxi_r = !self.auxi_r},
            //  Left Shift
            14 => { self.auxi_r = self.auxi_r << 1; },
            //  Right Shift
            15 => { self.auxi_r = self.auxi_r >> 1; },

            // Aux/tape operations
            //  OR
            16 => { self.data_tape[self.data_p as usize] =
                    self.data_tape[self.data_p as usize] | self.auxi_r; },
            //  AND
            17 => { self.data_tape[self.data_p as usize] =
                    self.data_tape[self.data_p as usize] & self.auxi_r; },
            //  XOR
            18 => { self.data_tape[self.data_p as usize] =
                    self.data_tape[self.data_p as usize] ^ self.auxi_r; },
            //  NOR
            19 => { self.data_tape[self.data_p as usize] =
                    !(self.data_tape[self.data_p as usize] | self.auxi_r); },
            //  NAND
            20 => { self.data_tape[self.data_p as usize] =
                    !(self.data_tape[self.data_p as usize] & self.auxi_r); },
            //  ADD
            21 => { self.data_tape[self.data_p as usize] =
                    self.data_tape[self.data_p as usize].wrapping_add(self.auxi_r); },
            //  DIFFERENCE
            22 => { self.data_tape[self.data_p as usize] =
                    self.data_tape[self.data_p as usize].wrapping_sub(self.auxi_r); },
            //  QUOTIENT
            23 => { self.data_tape[self.data_p as usize] =
                    self.data_tape[self.data_p as usize].wrapping_div(self.auxi_r); },
            //  MODULO
            24 => { self.data_tape[self.data_p as usize] =
                    self.data_tape[self.data_p as usize] % self.auxi_r; },
            //  PRODUCT
            25 => { self.data_tape[self.data_p as usize] =
                    self.data_tape[self.data_p as usize].wrapping_mul(self.auxi_r); },
            31 => { return FlowAction::Done; },
            _ => {},
        }
        return FlowAction::NoAction;
}
}




