use instructions::OpCode;

pub fn tokenize_line(source: &str) -> Result<(OpCode, usize), &'static str> {
    let mut tokens = source.split_whitespace().take(2);
    let opcode_src = tokens.next();
    let address_src = tokens.next();

    let opcode = match opcode_src {
        Some(s) => match s {
            "LOAD" => Ok(OpCode::Load),
            "ADD" => Ok(OpCode::Add),
            "SUB" => Ok(OpCode::Sub),
            "STORE" => Ok(OpCode::Store),
            "JGZ" => Ok(OpCode::Jgz),
            "READ" => Ok(OpCode::Read),
            "PRINT" => Ok(OpCode::Print),
            "HALT" => Ok(OpCode::Halt),
            _ => Err("Invalid OpCode")
        },
        None => Err("Missing OpCode")
    };

    let address = match address_src {
        Some(s) => match s.parse::<usize>() {
            Ok(addr) => Ok(addr),
            Err(_) => Err("Invalid Argument")
        },
        None => Err("Missing Argument")
    };

    match (opcode, address) {
        (Err(msg), _) => Err(msg),
        (_, Err(msg)) => Err(msg),
        (opcode, address) => Ok((opcode.unwrap(), address.unwrap()))
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use instructions::OpCode;

    macro_rules! assert_match {
        ($x:expr, $p:pat) => {
            match $x {
                $p => (),
                _ => panic!("{} doesn't match {}", stringify!($x), stringify!($p))
            }
        }
    }

    #[test]
    fn test_tokenize_line(){
        // Check that correct tokens all parse
        assert_match!(tokenize_line("LOAD 1"),
                      Ok((OpCode::Load, 1)));
        assert_match!(tokenize_line("ADD 2"),
                      Ok((OpCode::Add, 2)));
        assert_match!(tokenize_line("SUB 3"),
                      Ok((OpCode::Sub, 3)));
        assert_match!(tokenize_line("STORE 4"),
                      Ok((OpCode::Store, 4)));
        assert_match!(tokenize_line("JGZ 5"),
                      Ok((OpCode::Jgz, 5)));
        assert_match!(tokenize_line("READ 6"),
                      Ok((OpCode::Read, 6)));
        assert_match!(tokenize_line("PRINT 7"),
                      Ok((OpCode::Print, 7)));
        assert_match!(tokenize_line("HALT 8"),
                      Ok((OpCode::Halt, 8)));

        // Check that invalid usizes don't parse
        assert_match!(tokenize_line("ADD -1"),
                      Err("Invalid Argument"));

        // Check that invalid tokens throw errors
        assert_match!(tokenize_line("FOO 3"),
                      Err("Invalid OpCode"));

        // Check that missing arguments throw errors
        assert_match!(tokenize_line("HALT"),
                      Err("Missing Argument"));

        // Check that empty lines are invalid
        assert_match!(tokenize_line(""),
                      Err("Missing OpCode"));
    }

}
