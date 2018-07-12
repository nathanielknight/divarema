# The DIdactic VAnity REgister MAchine

This is a pet-project interpreter which implements a random access
machine as described by
[Cook and Reckhow](https://www.cs.toronto.edu/~sacook/homepage/rams.pdf).

`divarema` only implements the machine's basic instructions; the paper
linked above also includes a proof regarding a VM built with these
instructions which can execute arbitrary programs stored in main
memory within some time bounds (so an arbitrarily large program can be
executed with a space-bounded instruction and ROM program).


## Motivation

This is a pet project to help me learn about models of computation and
about Rust (hence didactic and vanity). It's not intended for **any**
other application, but if you get something from it that's great :)


## The Machine

`divarema` reads instructions form a file and executes them. The
machine has eight instructions and two kinds of memory. Program memory
stores the program being executed and is immutable. Main memory is
available to the program and consists of a number (by default, 8) of
mutable registers that can hold signed 32-bit integers. Main memory is
indexed by integers, starting from 0.

Two special registers control operation and implement arithmetic:

- The accumulator `ACC` is the target and source of arithmetic and copy
  operations
- The instruction counter `IC` holds the address (in program-memory)
  of the next instruction to execute, manipulated by control-flow
  operations


## Instruction Set

Programs for `divarema` can include the following instructions:

Operation | Description                            | Mnemonic                
--------- | -------------------------------------- | -------------------------
LOAD j    | ACC <- j ; IC <- IC+2                  | load                    
ADD j     | ACC <- [ACC] + [Xj] ; IC <- IC+2       | add                     
SUB j     | ACC <- [ACC] - [Xj] ; IC <- IC+2       | subtract                
STORE j   | Xj <- [ACC] ; IC <- IC+2               | store                   
JGZ j     | if AC > 0 then IC <- j else IC <- IC+2 | jump-greather-than-zero 
READ j    | Xj <- next input                       | read                    
PRINT j   | output [Xj]                            | print                   
HALT _    | stop execution                         | halt                    


## License

This software is licensed under the GNU General Public License
v3.0. See `LICENSE`
or [license's website](https://www.gnu.org/licenses/gpl-3.0.en.html)
for details.
