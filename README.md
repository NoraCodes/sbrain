# libsbrain
A library for execution of Semantic Brain, based on Urban Müller's famous but unprintable language.

If you're looking for a compiler/interpreter standalone, it's called [sbic](https://github.com/silverwingedseraph/sbic).

 ## What is SBrain?
   SBrain, or Semantic Brain, is a language based on Urban Müller's famous language with only 8 symbols (3 bit instructions).
   SBrain's additions increase the number of symbols to 16 (4 bit instructions) and adds a stack and a register.

 ## Examples


 ## Specification
 ### Data Structures
   SBrain requires:

 * a read/write **data tape** which is addressable up to, at minimum, 65,536 (0x0 - 0xFFFF) 8-bit cells. They must be initially set to zero.
 * a read/write **data stack** with room for at least 256 values. They must be initially set to zero.
 * a read-only tape which contains the executable code. This code is represented as a list of unsigned integers of, at minimum, six bits in width.
 * a read-only nonreversable tape containing the program's input (a function like `getch()` works fine.)
 * a write-only nonreversable tape containing the program's output (a function like `putch()` works fine.)
 * a read/write register (`data_p`) of enough bits to store a position on the data tape
 * a read/write register (`inst_p`) of enough bits to store a position on the instruction tape
 * a read/write register (`auxi_r`) of the same size as a cell on the data tape

 ### Commands and Source Code

 SBrain source code consists of text characters. Executable code consists of unsigned integers of six bits. A transliterator converts the source code to executable code by a one-to-one mapping, with one exception: all data between # characters, including those characters, is ignored by the transliterator.

 The first eight instructions are the standard brainf--- instructions. **Any brainf--- program is a valid SBrain program and should behave in the same way as in a standard, semantically equivalent brainf--- interpreter**, so long as comments are properly escaped.

 Decimal | Code  | Semantics
 --------|-------|----------
        0|      <| Decrement `data_p`
        1|      >| Increment `data_p`
        2|      -| Subtract one from the cell pointed at by `data_p`
        3|      +| Add one to the cell pointed at by `data_p`
        4|      [| If the cell pointed at by `data_p` is zero, move `inst_p` to point to the matching `]`, plus one.
        5|      ]| If the cell pointed at by `data_p` is nonzero, move `inst_p` to point to the matching `]`, plus one.
        6|      .| Place the value in the cell pointed at by `data_p` on the output tape
        7|      ,| Place the next value from the input tape in the cell pointed at by `data_p`
        8|      {| Push the value from the cell pointed at by `data_p` onto the stack
        9|      }| Pop the next value from the stack into the cell pointed at by `data_p`
       10|      (| Set `auxi_r` to the value of the cell pointed at by `data_p`
       11|      )| Set the cell pointed at by `data_p` to the value in `auxi_r`
       12|      ^| Set the value in `auxi_r` to 0
       13|      !| Perform a bitwise NOT on the value in `auxi_r`.
       14|      &| Perform a bitwise AND on the value in `auxi_r` and the cell pointed at `data_p`, placing the value in `auxi_r`.
       15|      @| End the program. The exit code is the value in `auxi_r`. 

 ### Further Rules
 No read operation shall ever disrupt a cell on the data tape.
 
 Reading an EOF always produces a 0.
 
 Non-command characters in the instruction section of source code must be ignored.
 
 In the case of the instruction pointer running off the end of the tape, it must wrap to the
 beginning.


