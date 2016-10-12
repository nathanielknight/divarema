use instructions::{OpCode, DivaremaProgram};

type DivaremaMemory = Vec<i32>;

#[derive(Debug,PartialEq)]
pub struct DivaremaEngine {
    program: DivaremaProgram,
    memory: DivaremaMemory,
    acc: i32,
    instruction_counter: usize,
}


impl DivaremaEngine {

    fn new(program: DivaremaProgram, memsize: usize) -> DivaremaEngine{
        DivaremaEngine {
            program: program,
            memory: vec![0; memsize],
            acc: 0,
            instruction_counter: 0,
        }
    }

    fn apply(&mut self, action: (OpCode, usize)) {
        let (op, arg) = action;

        let executor = match op {
            OpCode::Load => apply_load(self, arg),
            OpCode::Add => apply_add(self, arg),
            OpCode::Sub => apply_sub(self, arg),
            OpCode::Store => apply_store(self, arg),
            OpCode::Jgz => unimplemented!(),
            OpCode::Read => unimplemented!(),
            OpCode::Print => unimplemented!(),
            OpCode::Halt => unimplemented!()
        };
    }
    
}


fn apply_load(engine: &mut DivaremaEngine, arg: usize) {
    let val = engine.memory[arg];
    let next_ic = engine.instruction_counter + 1;
    engine.acc = val;
    engine.instruction_counter = next_ic;
}

#[test]
fn test_apply_load() {
    let mut test_engine = DivaremaEngine::new(vec![], 10);
    assert_eq!(test_engine.acc, 0);
    test_engine.memory[3] = 9;
    apply_load(&mut test_engine, 3);
    assert_eq!(test_engine.acc, 9);
}


fn apply_add(engine: &mut DivaremaEngine, arg: usize) {
    let val = engine.memory[arg];
    let next_ic = engine.instruction_counter + 1;
    engine.acc = engine.acc + val;
    engine.instruction_counter = next_ic;
}

#[test]
fn test_apply_add() {
    let mut test_engine = DivaremaEngine::new(vec![], 10);
    test_engine.memory[0] = 1;
    test_engine.memory[9] = 2;
    apply_add(&mut test_engine, 0);
    assert_eq!(test_engine.acc, 1);
    apply_add(&mut test_engine, 9);
    assert_eq!(test_engine.acc, 3);
}

fn apply_sub(engine: &mut DivaremaEngine, arg: usize) {
    let val = engine.memory[arg];
    let next_ic = engine.instruction_counter + 1;
    engine.acc = engine.acc - val;
    engine.instruction_counter = next_ic;
}

#[test]
fn test_apply_sub() {
    let mut test_engine = DivaremaEngine::new(vec![], 10);
    test_engine.memory[0] = 1;
    test_engine.memory[9] = 2;
    apply_sub(&mut test_engine, 0);
    assert_eq!(test_engine.acc, -1);
    apply_sub(&mut test_engine, 9);
    assert_eq!(test_engine.acc, -3);
}


fn apply_store(engine: &mut DivaremaEngine, arg: usize) {
    engine.memory[arg] = engine.acc;
    engine.instruction_counter += 1;
}

#[test]
fn test_apply_store() {
    let mut test_engine = DivaremaEngine::new(vec![], 10);
    test_engine.acc = 5;
    apply_store(&mut test_engine, 0);
    assert_eq!(test_engine.memory[0], 5);
    test_engine.acc = 7;
    apply_store(&mut test_engine, 9);
    assert_eq!(test_engine.memory[9], 7);
}


#[cfg(test)]
mod tests {
    use super::*;
    use instructions::{OpCode, DivaremaProgram};

    #[test]
    fn test_engine_init() {
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
            10
        );

        let test_engine_raw = DivaremaEngine {
            program: test_prog,
            memory: vec![0; 10],
            acc: 0,
            instruction_counter: 0
        };

        assert_eq!(test_engine, test_engine_raw);
    }


}
