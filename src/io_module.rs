//! Formatted input and output for a Divarema engine
//!
//! This module implements a formatted reader and writer for the
//! DivaremaEngine.
//!
//! ## Conversion between numbers and strings
//!
//! Because the engine is didactic, ints read or
//! written should be as accessible as possible for users of the
//! engine. For this reason, when the engine prints a number it will
//! be sent as an ASCII encoded, newline terminated sequence of bytes.
//! When the engine reads a number, it will read it as an ASCII
//! encoded, newline terminated squence of bytes.
//!
//! The conversion is performed according to this little grammar:
//!
//! ```text
//! int : (-)?(digits)+(\n)
//! digits : 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 0
//! ```

// These are internal

use std::str::from_utf8;
use std::io::{BufRead, BufReader, Write};

fn encode_digit(x: i32) -> u8 {
    match x {
        0 => '0' as u8,
        1 => '1' as u8,
        2 => '2' as u8,
        3 => '3' as u8,
        4 => '4' as u8,
        5 => '5' as u8,
        6 => '6' as u8,
        7 => '7' as u8,
        8 => '8' as u8,
        9 => '9' as u8,
        _ => panic!("Error encoding digit: {}", x)
    }
}

fn format_i32(x: i32) -> Vec<u8> {
    let mut cs: Vec<u8> = Vec::new();
    if x == 0 {
        cs.push('0' as u8);
        return cs
    }
    if x < 0 {
        let mut x = x * -1;
        while x > 0 {
            let min_digit = x % 10;
            cs.push(encode_digit(min_digit));
            x = x / 10;
        }
        cs.push('-' as u8);
    } else {
        let mut x = x;
        while x > 0 {
            let min_digit = x % 10;
            cs.push(encode_digit(min_digit));
            x = x / 10;
        }
    }
    cs.reverse();
    cs
}

#[test]
fn test_format_i32() {
    let repr_3 = vec!['3' as u8];
    assert_eq!(format_i32(3), repr_3);
    let repr_567 = b"567";
    assert_eq!(format_i32(567), repr_567);
    let repr_n91 = b"-91";
    assert_eq!(format_i32(-91), repr_n91);
}


fn parse_i32(xs: Vec<u8>) -> i32 {
    let cs: &[u8] = &xs[..];
    let s = from_utf8(cs).unwrap();
    let n = s.parse::<i32>().unwrap();
    n
}

#[test]
fn test_parse_i32() {
    let repr_3 = b"3".to_vec();
    assert_eq!(3, parse_i32(repr_3));
    let repr_78129 = b"78129".to_vec();
    assert_eq!(78129, parse_i32(repr_78129));
    let repr_n11 = b"-11".to_vec();
    assert_eq!(-11, parse_i32(repr_n11));
}


#[test]
fn test_parse_and_format_i32() {
    for i in -9999..9999 {
        assert_eq!(format_i32(i),
                   format_i32(parse_i32(format_i32(i))));
    }
}


/// A `DivaremaIoModule` has two fields: an input and an output, which
/// can be anything that implemented `BufRead` and `Write`
/// respectively. The DivaremaEngine that the IO Module belongs to
/// should be the only thing that accesses these directly; the module
/// has no public methods.
///
/// # Examples
///
/// ```rust
/// use divarema::io_module::DivaremaIoModule;
/// use std::io::{stdin, BufReader, stdout};
/// let module1 = DivaremaIoModule{input: BufReader::new(stdin()), output: stdout()};
/// ```
///
/// A socket or a file should work just as well.

#[derive(Debug)]
pub struct DivaremaIoModule<I: BufRead, O: Write> {
    pub input: I,
    pub output: O
}

impl<I: BufRead, O: Write> DivaremaIoModule<I,O> {

    pub fn get_int(&mut self) -> Result<i32, &'static str> {
        let mut buf = String::new();
        let r = self.input.read_line(&mut buf);
        match r {
            Ok(l) => {
                buf.truncate(l-1);
                Ok(parse_i32(buf.into_bytes()))
            },
            Err(msg) => {
                println!("Error reading '{}'", buf);
                Err("Couldn't parse integer")
            }
        }
    }

    pub fn put_int(&mut self, x: i32) -> Result<(), &str> {
        let mut cs = format_i32(x);
        cs.push('\n' as u8);
        self.output.write(&cs);
        self.output.flush().map_err(|c| "Write error")
    }
}


#[test]
fn test_io_module() {
    let inp: &[u8] = &String::from("123\n-456\n").into_bytes();
    let mut outp: Vec<u8> = Vec::new();

    let mut module1 = DivaremaIoModule {
        input: BufReader::new(inp),
        output: outp,
    };

    let x = module1.get_int().unwrap();
    assert_eq!(x, 123);

    module1.put_int(789);
    assert_eq!(module1.output, b"789\n");

    let x = module1.get_int().unwrap();
    println!("{}", x);
    assert_eq!(x, -456);

    module1.put_int(-30);
    assert_eq!(module1.output, b"789\n-30\n");
}
