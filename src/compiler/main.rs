mod tokenizer;
mod compilation;

use crate::tokenizer::*;
use crate::compilation::*;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn main() {
    if env::args().len() < 3 {
        println!("Incorrect command usage");
        println!("Propper usage is as follows: ");
        println!("vertc <input.vt> <output>");
        std::process::exit(1);
    }
    let argv: Vec<String> = env::args().collect();
    let src_file = File::open(&argv[1]).expect("Failed to open source file");
    let mut buf_reader = BufReader::new(src_file);
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    let tokens = tokenize(&contents);
    let mut compiled_code: Vec<u8> = vec![];
    if argv.len() > 3 {
        compiled_code = tokens_to_vm(tokens);
    }
    else {
        compiled_code = tokens_to_py(tokens);
    }
    let mut output_file = File::create(&argv[2]).expect("Failed to open output file");
    let _ = output_file.write_all(&compiled_code);
}
