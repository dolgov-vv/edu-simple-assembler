#![allow(dead_code)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod asmcore;

use std::{env, fs};
use std::io::{Write, stdout};
use text_io::read;
use asmcore::*;

fn main() {
    if let Some(script_name) = get_script_name() {
        let prg = fs::read_to_string(script_name).expect("Can't read assembly script file");
        let program = AssemblerParser::parse_program(prg.as_str()).expect("Syntax error");
        let mut process = ExecutingContext::new(&program);
        process.execute();
    }
}

fn get_script_name() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        args.get(1).cloned()
    } else {
        print!("Enter script file name: ");
        stdout().flush().ok();
        let script_name: String = read!();
        Some(script_name)
    }
}