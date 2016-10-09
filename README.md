# The DIdactic VAnity REgister MAchine

This is a pet-project bytecode interpreter which implements a random
access machine after the
[model of Cook and Reckhow](https://www.cs.toronto.edu/~sacook/homepage/rams.pdf).

`divarema` only implements the machine's basic instructions; the paper
linked above also includes a proof regarding a VM built with these
instructions which can execute arbitrary programs stored in main
memory within some time bounds (so an arbitrarily large program can be
executed with a space-bounded instruction and ROM program). I might
implement that as a demonstration, but that's for future work.


## Motivation

This is a pet project to help me learn about models of computation and
about Rust (hence didactic and vanity). It's not intended for **any**
other application, but if you get something from it that's great :)


## Memory model

The `diverema`'s program memory is static, and created at load
time. It holds the program as decoded from the source.

Two special registers control operation and implement arithmetic:

- The acumulator `ACC` is the target and source of arithmetic and copy
  operations
- The instruction counter `IC` holds the address (in program-memory)
  of the next instruction to execute, manipulated by control-flow
  operations (and trivially manipulated by 
  
The `divarema`'s main memory is statically allocated as an array of
integers, the size of which is set when the machine starts.  Registers
each have theoretically infinite capacity, but practically are
implemented as `i32`'s for simplicity's sake. They are initialized to
zero. Registers are addressed by a positive integer, starting with 0.


## Instruction Set

Program instructions include:

<pre>
| Operation | Description                            | OpCode | Mnemonic                |
|-----------+----------------------------------------+--------+-------------------------|
| LOAD j    | ACC <- j ; IC <- IC+2                  |      1 | load                    |
| ADD j     | ACC <- [ACC] + [Xj] ; IC <- IC+2       |      2 | add                     |
| SUB j     | ACC <- [ACC] - [Xj] ; IC <- IC+2       |      3 | subtract                |
| STORE j   | Xj <- [ACC] ; IC <- IC+2               |      4 | store                   |
| JGZ j     | if AC > 0 then IC <- j else IC <- IC+2 |      5 | jump-greather-than-zero |
| READ j    | Xj <- next input                       |      6 | read                    |
| PRINT j   | output [Xj]                            |      7 | print                   |
| HALT      | stop execution                         |      _ | halt                    |
</pre>
