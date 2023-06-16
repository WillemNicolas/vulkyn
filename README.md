# vulkyn
This project is a simple implementation of a stack-based virtual machine (VM) written in Rust. The VM includes a stack, a heap, and several registers, and it interprets instructions written in its own assembly language.

## Features
  - Stack-based architecture: The VM operates using a stack data structure to store and manipulate values.
  - Heap memory: The VM provides a heap for dynamically allocating memory.
  - Register set: The VM includes a set of registers for storing and accessing values.
  - Assembly language: The VM has its own assembly language for writing programs and executing instructions.

## Getting Started
To use the virtual machine, follow these steps:
  - Ensure you have Rust installed.
    
    You can download it from the official Rust website: https://www.rust-lang.org/.
  - Clone the project repository :
    ```console
      git clone git@github.com:WillemNicolas/vulkyn.git
    ```
  - Build  :
    ```console
      cargo build
    ```
## Assembly Language

The virtual machine uses a simple assembly language that consists of instructions and operands. Instructions are represented as strings of text, and operands are values or registers that the instructions operate on.

Here are the supported instructions:
### MEMORY ACCESS 
  - PUSH, ( one parameters, either a word or register)
  - POP, ( no parameters )
  - SCOPY, ( one parameter, a register)
  - SMOVE, ( one parameter, a register)
  - RCOPY, ( two parameters, both registers)
  - RMOVE, ( two parameters, both registers)
  - RWRITE, ( two parameters, a register and a word)
  - LOAD, ( an address )
  - LOADB, ( two parameters an address and a u64)
  - READU, ( three  parameters an address and two u64)
  - READD, ( three  parameters an address and two u64)
  - SREADU, ( two parameters, both u64)
  - SREADD, ( two parameters, both u64)
  - WRITE, ( two parameters, a word and an address)
  - SWRITE, ( no parameters )
  - ALLOC, ( one parameters, a u64)
  - FREE, ( an address )
  - SFREE, ( no parameters )
 ### OPERATOR
   > \+
  - ADD, ( no parameters )
  - RADD, ( two parameters, both either a word or register)
   > \-
  - MINUS, ( no parameters )
  - RMINUS, ( two parameters, both either a word or register)
   > \*
  - MUL, ( no parameters )
  - RMUL, ( two parameters, both either a word or register)
   > \/
  - DIV, ( no parameters )
  - RDIV, ( two parameters, both either a word or register)
   > \% 
  - MOD, ( no parameters )
  - RMOD, ( two parameters, both either a word or register)
   > \&
  - BAND, ( no parameters )
  - RBAND, ( two parameters, both either a word or register)
   > \|
  - BOR, ( no parameters )
  - RBOR, ( two parameters, both either a word or register)
   > \^
  - BXOR, ( no parameters )
  - RBXOR, ( two parameters, both either a word or register)
   > \>>
  - RSHIFT, ( no parameters )
  - RRSHIFT, ( two parameters, both either a word or register)
   > \<<
  - LSHIFT, ( no parameters )
  - RLSHIFT, ( two parameters, both either a word or register)
   > \== 
  - EQUAL, ( no parameters )
  - REQUAL, ( two parameters, both either a word or register)
   > \!=
  - DIFF, ( no parameters )
  - RDIFF, ( two parameters, both either a word or register)
   > \! 
  - NOT, ( no parameters )
  - RNOT, ( one parameters, either a word or register)
   > \&&
  - AND, ( no parameters )
  - RAND, ( two parameters, both either a word or register)
   > \||
  - OR, ( no parameters )
  - ROR, ( two parameters, both either a word or register)
   > \<
  - LESS, ( no parameters )
  - RLESS, ( two parameters, both either a word or register)
   > \<=
  - ELESS, ( no parameters )
  - RELESS, ( two parameters, both either a word or register)
   > \>
  - GREAT, ( no parameters )
  - RGREAT, ( two parameters, both either a word or register)
   > \>=
  - EGREAT, ( no parameters )
  - REGREAT, ( two parameters, both either a word or register)
 ### Conversion
  - F2I, ( no parameters )
  - F2U, ( no parameters )
  - F2B, ( no parameters )
  - F2C, ( no parameters )
  - RF2I, ( one parameters, either a word or register)
  - RF2U, ( one parameters, either a word or register)
  - RF2B, ( one parameters, either a word or register)
  - RF2C, ( one parameters, either a word or register)
  - I2F, ( no parameters )
  - I2U, ( no parameters )
  - I2B, ( no parameters )
  - I2C, ( no parameters )
  - RI2F, ( one parameters, either a word or register)
  - RI2U, ( one parameters, either a word or register)
  - RI2B, ( one parameters, either a word or register)
  - RI2C, ( one parameters, either a word or register)
  - U2I, ( no parameters )
  - U2F, ( no parameters )
  - U2C, ( no parameters )
  - U2B, ( no parameters )
  - RU2I, ( one parameters, either a word or register)
  - RU2F, ( one parameters, either a word or register)
  - RU2C, ( one parameters, either a word or register)
  - RU2B, ( one parameters, either a word or register)
  - C2I, ( no parameters )
  - C2F, ( no parameters )
  - C2U, ( no parameters )
  - C2B, ( no parameters )
  - RC2I, ( one parameters, either a word or register)
  - RC2F, ( one parameters, either a word or register)
  - RC2U, ( one parameters, either a word or register)
  - RC2B, ( one parameters, either a word or register)
  - B2I, ( no parameters )
  - B2F, ( no parameters )
  - B2U, ( no parameters )
  - B2C, ( no parameters )
  - RB2I, ( one parameters, either a word or register)
  - RB2F, ( one parameters, either a word or register)
  - RB2U, ( one parameters, either a word or register)
  - RB2C, ( one parameters, either a word or register)
 ### PRINT
  - DMP, ( no parameters )
  - RDMP, ( one parameters, either a word or register)
 ### FLOW 
  - EXIT, ( no parameters )
  - NOP, ( no parameters )
  - LABEL, ( no parameters )
  - GO, ( one parameters, a u64)
  - GOIF, ( one parameters, a u64)
  - RGOIF, ( two parameters, a u64 and a register)
  - CALL, ( one parameters, a u64)
  - SCALL, ( no parameters )
  - CALLP, ( two parameters, both u64)
  - SCALLP, ( one parameters, a u64)
  - RCALL, ( one parameter, a register)
  - RCALLP, ( two parameters, a register and a u64)
  - RET, ( one parameters, a u64)
