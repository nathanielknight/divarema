use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;

extern crate clap;
use clap::{App, Arg};

mod engine;
mod instructions;
mod io_module;
mod parser;


const MEMSIZE: usize = 8;


fn main() {
    let matches =  App::new("DiVaReMa")
        .version("0.1.0")
        .author("Nathaniel KNight <nathaniel.ep@gmail.com>")
        .about("The Didactic Vanity Register Machine")
        .arg(Arg::with_name("progfile")
             .help("File-name of the source code to interpret")
             .required(true)
             .index(1))
        .get_matches();

    // Load the source file
    let fname = matches.value_of("progfile").unwrap(); // unwrap is safe bc. arg is required
    let mut file = File::open(fname).expect("Unable to open file");
    let mut source = String::new();
    file.read_to_string(&mut source).expect("Unable read file");


    // Parse the source code
    let program = match parser::tokenize(&source) {
        Ok(prog) => prog,
        Err((ln, msg)) => panic!(format!("Error on line {}: {}", ln, msg))
    };

    // Set up the execution environment
    let stdout = io::stdout();
    let stdin = BufReader::new(io::stdin());
    let mut prog_engine = engine::DivaremaEngine::new(
        program, MEMSIZE, stdin, stdout);

    // Execute
    prog_engine.run();
    
}
