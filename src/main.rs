mod tokenizer;

use crate::tokenizer::*;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;



fn tokens_to_py(tokens: Vec<Token>) -> String {
    let mut code: Vec<String> = vec![
"stack = []".to_string(),
    ];
    let mut stack: Vec<String> = vec![];
    for token in tokens {
        match token.ttype {
            TokenType::TStrLit => stack.push(format!("\"{}\"",token.value.unwrap())),
            TokenType::TPrint => code.push("print(stack.pop())".to_string()),
            TokenType::TSemi => {
                while stack.len() >= 1 {
                    code.push(format!("stack.append({})", stack.pop().unwrap()))
                }
            },
        }
    }
    code.join("\n")
}

fn main() {
    if env::args().len() < 2 {
        println!("Incorrect command usage");
        println!("Propper usage is as follows: ");
        println!("vertigo <input.vt> <output>");
        std::process::exit(1);
    }
    let argv: Vec<String> = env::args().collect();
    let src_file = File::open(&argv[1]).expect("Failed to open source file");
    let mut buf_reader = BufReader::new(src_file);
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    let tokens = tokenize(&contents);
    let compiled_code = tokens_to_py(tokens);
    let mut output_file = File::create(&argv[2]).expect("Failed to open output file");
    let _ = output_file.write_all(compiled_code.as_bytes());
}
