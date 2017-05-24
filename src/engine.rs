use std::io::{BufRead, BufReader, Write, sink, empty};
use std::cmp::PartialEq;

use instructions::{OpCode, DivaremaProgram};
use io_module::DivaremaIoModule;

type DivaremaMemory = Vec<i32>;

#[derive(Debug)]
pub struct DivaremaEngine<I: BufRead, O: Write> {
    program: DivaremaProgram,
    memory: DivaremaMemory,
    acc: i32,
    instruction_counter: usize,
    io_module: DivaremaIoModule<I,O>
}


impl<I: BufRead, O: Write> PartialEq for DivaremaEngine<I,O> {
    /// Divarema engines are considered equal if their programs,
    /// memories, accumulators, and instruction counters match. We
    /// ignore their IO engines.
    fn eq(&self, other: &DivaremaEngine<I,O>) -> bool {
        self.program == other.program &&
            self.memory == other.memory &&
            self.acc == other.acc &&
            self.instruction_counter == other.instruction_counter
    }
}


impl<I: BufRead, O: Write> DivaremaEngine<I,O> {

    fn new(program: DivaremaProgram, memsize: usize, inp: I, outp: O) -> DivaremaEngine<I,O>{
        DivaremaEngine {
            program: program,
            memory: vec![0; memsize],
            acc: 0,
            instruction_counter: 0,
            io_module: DivaremaIoModule{input: inp, output: outp},
        }
    }

    fn apply(&mut self, action: (OpCode, usize)) {
        let (op, arg) = action;

        let executor = match op {
            OpCode::Load => apply_load(self, arg),
            OpCode::Add => apply_add(self, arg),
            OpCode::Sub => apply_sub(self, arg),
            OpCode::Store => apply_store(self, arg),
            OpCode::Jgz => apply_jgz(self, arg),
            OpCode::Read => unimplemented!(),
            OpCode::Print => unimplemented!(),
            OpCode::Halt => unimplemented!()
        };
    }

}


fn apply_load<I: BufRead, O: Write>(engine: &mut DivaremaEngine<I,O>, arg: usize) {
    let val = engine.memory[arg];
    let next_ic = engine.instruction_counter + 1;
    engine.acc = val;
    engine.instruction_counter = next_ic;
}

#[test]
fn test_apply_load() {
    let inp = empty();
    let outp = sink();
    let mut test_engine = DivaremaEngine::new(vec![], 10, inp, outp);
    assert_eq!(test_engine.acc, 0);
    test_engine.memory[3] = 9;
    apply_load(&mut test_engine, 3);
    assert_eq!(test_engine.acc, 9);
}


fn apply_add<I: BufRead, O: Write>(engine: &mut DivaremaEngine<I,O>, arg: usize) {
    let val = engine.memory[arg];
    let next_ic = engine.instruction_counter + 1;
    engine.acc = engine.acc + val;
    engine.instruction_counter = next_ic;
}

#[test]
fn test_apply_add() {
    let inp = empty();
    let outp = sink();
    let mut test_engine = DivaremaEngine::new(vec![], 10, inp, outp);
    test_engine.memory[0] = 1;
    test_engine.memory[9] = 2;
    apply_add(&mut test_engine, 0);
    assert_eq!(test_engine.acc, 1);
    apply_add(&mut test_engine, 9);
    assert_eq!(test_engine.acc, 3);
}


fn apply_sub<I: BufRead, O: Write>(engine: &mut DivaremaEngine<I,O>, arg: usize) {
    let val = engine.memory[arg];
    let next_ic = engine.instruction_counter + 1;
    engine.acc = engine.acc - val;
    engine.instruction_counter = next_ic;
}

#[test]
fn test_apply_sub() {
    let inp = empty();
    let outp = sink();
    let mut test_engine = DivaremaEngine::new(vec![], 10, inp, outp);
    test_engine.memory[0] = 1;
    test_engine.memory[9] = 2;
    apply_sub(&mut test_engine, 0);
    assert_eq!(test_engine.acc, -1);
    apply_sub(&mut test_engine, 9);
    assert_eq!(test_engine.acc, -3);
}


fn apply_store<I: BufRead, O: Write>(engine: &mut DivaremaEngine<I,O>, arg: usize) {
    engine.memory[arg] = engine.acc;
    engine.instruction_counter += 1;
}

#[test]
fn test_apply_store() {
    let inp = empty();
    let outp = sink();
    let mut test_engine = DivaremaEngine::new(vec![], 10, inp, outp);
    test_engine.acc = 5;
    apply_store(&mut test_engine, 0);
    assert_eq!(test_engine.memory[0], 5);
    test_engine.acc = 7;
    apply_store(&mut test_engine, 9);
    assert_eq!(test_engine.memory[9], 7);
}


fn apply_jgz<I: BufRead, O: Write>(engine: &mut DivaremaEngine<I,O>, arg: usize) {
    if engine.acc > 0 {
        engine.instruction_counter = arg;
    } else {
        engine.instruction_counter += 1;
    }
}


#[test]
fn test_apply_jgz() {
    let inp = empty();
    let outp = sink();
    let mut test_engine = DivaremaEngine::new(vec![], 10, inp, outp);
    test_engine.acc = 1;
    apply_jgz(&mut test_engine, 9);
    assert_eq!(test_engine.instruction_counter, 9);
    test_engine.instruction_counter = 0;
    test_engine.acc = -1;
    apply_jgz(&mut test_engine, 9);
    assert_eq!(test_engine.instruction_counter, 1);
}


fn apply_read<I: BufRead, O: Write>(engine: &mut DivaremaEngine<I,O>, arg: usize) {
    let next_ic = engine.instruction_counter + 1;
    let n = engine.io_module.get_int().unwrap();
    engine.memory[arg] = n;
    engine.instruction_counter = next_ic;
}

#[test]
fn test_apply_read() {
    let inp: &[u8] = &String::from("123\n-456\n").into_bytes();
    let outp = sink();
    let mut test_engine = DivaremaEngine::new(vec![], 10, inp, outp);

    apply_read(&mut test_engine, 0);
    assert_eq!(test_engine.memory[0], 123);
    apply_read(&mut test_engine, 1);
    assert_eq!(test_engine.memory[1], -456);
}


fn apply_write<I: BufRead, O: Write>(engine: &mut DivaremaEngine<I,O>, arg: usize) {
    let next_ic = engine.instruction_counter + 1;
    let outp_n = engine.memory[arg];
    engine.io_module.put_int(outp_n);
    engine.instruction_counter = next_ic;
}

#[test]
fn test_apply_write() {
    let inp = empty();
    let mut outp: Vec<u8> = Vec::new();
    let mut test_engine = DivaremaEngine::new(vec![], 10, inp, outp);
    test_engine.memory[5] = 123;
    test_engine.memory[6] = -456;

    apply_write(&mut test_engine, 5);
    assert_eq!(test_engine.io_module.output, [49u8, 50u8, 51u8, 10u8]);
    apply_write(&mut test_engine, 6);
    assert_eq!(
        test_engine.io_module.output,
        [49u8, 50u8, 51u8, 10u8,
         45u8, 52u8, 53u8, 54u8, 10u8]);
}


#[cfg(test)]
mod tests {
    use super::*;
    use instructions::{OpCode, DivaremaProgram};

    #[test]
    fn test_engine_init() {
        let inp = empty();
        let outp = sink();
        let test_prog = vec![
            (OpCode::Load, 0),
            (OpCode::Print, 0),
            (OpCode::Halt, 0),
        ];

        let test_engine = DivaremaEngine::new(
            vec![
                (OpCode::Load, 0),
                (OpCode::Print, 0),
                (OpCode::Halt, 0),
            ],
            10,
            inp,
            outp,
        );

        let inp = empty();
        let outp = sink();

        let test_engine_raw = DivaremaEngine {
            program: test_prog,
            memory: vec![0; 10],
            acc: 0,
            instruction_counter: 0,
            io_module: DivaremaIoModule{input: inp, output: outp},
        };

        assert_eq!(test_engine, test_engine_raw);
    }


}
