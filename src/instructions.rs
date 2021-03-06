#[derive(Debug,PartialEq,Clone)]
pub enum OpCode { Load, Add, Sub, Store, Jgz, Read, Print, Halt}

pub type DivaremaProgram = Vec<(OpCode, usize)>;
