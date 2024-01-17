mod tokenizer;

use crate::tokenizer::*;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;



fn tokens_to_py(tokens: Vec<Token>) -> String {
    enum State {
        Normal,
        Looping,
    }
    let mut state = State::Normal;
    let mut code: Vec<String> = vec![
"stack = []".to_string(),
    ];
    let mut stack: Vec<String> = vec![];
    let mut loop_stack: Vec<Vec<String>> = vec![];
    for token in tokens {
        match state {
            State::Normal => {
                match token.ttype {
                    TokenType::TStrLit => stack.push(format!("\"{}\"",token.value.unwrap())),
                    TokenType::TNumLit => stack.push(format!("{}",token.value.unwrap())),
                    TokenType::TPrint => code.push("print(stack.pop())".to_string()),
                    TokenType::TAsAscii => {
                        code.push("pop_str = stack.pop()".to_string());
                        code.push("for i in range(len(pop_str)):".to_string());
                        code.push("\tstack.append(ord(pop_str[i]))".to_string());
                    },
                    TokenType::TLoop => {
                        code.push("while True:".to_string());
                        loop_stack.push(vec![]);
                        state = State::Looping;
                    },
                    TokenType::TAdd => code.push("stack.append(stack.pop()+stack.pop())".to_string()),
                    TokenType::TSub => code.push("stack.append(stack.pop()-stack.pop())".to_string()),
                    TokenType::TMul => code.push("stack.append(stack.pop()*stack.pop())".to_string()),
                    TokenType::TDiv => code.push("stack.append(stack.pop()/stack.pop())".to_string()),
                    TokenType::TDup => {
                        code.push("dup = stack.pop()".to_string());
                        code.push("stack.append(dup)".to_string());
                        code.push("stack.append(dup)".to_string());
                    },
                    TokenType::TFlip => {
                        code.push("assert len(stack) >= 2".to_string());
                        code.push("tmp = stack[-2]".to_string());
                        code.push("stack[-2] = stack[-1]".to_string());
                        code.push("stack[-1] = tmp".to_string());
                    },
                    TokenType::TPop => code.push("stack.pop()".to_string()),
                    TokenType::TEnd => {
                        println!("Error ending outside of loop");
                        std::process::exit(1);
                    },
                    TokenType::TSemi => {
                        while stack.len() >= 1 {
                            code.push(format!("stack.append({})", stack.pop().unwrap()));
                        }
                    },
                }
            }
            State::Looping => {
                
                match token.ttype {
                    TokenType::TStrLit => stack.push(format!("\"{}\"",token.value.unwrap())),
                    TokenType::TNumLit => stack.push(format!("{}",token.value.unwrap())),
                    TokenType::TPrint => loop_stack.last_mut().unwrap().push("print(stack.pop())".to_string()),
                    TokenType::TAsAscii => {
                        loop_stack.last_mut().unwrap().push("pop_str = stack.pop()".to_string());
                        loop_stack.last_mut().unwrap().push("for i in range(len(pop_str)):".to_string());
                        loop_stack.last_mut().unwrap().push("\tstack.append(ord(pop_str[i]))".to_string());
                    },
                    TokenType::TLoop => {
                        loop_stack.last_mut().unwrap().push("while True:".to_string());
                        loop_stack.push(vec![]);
                        state = State::Looping;
                    },
                    TokenType::TAdd => loop_stack.last_mut().unwrap().push("stack.append(stack.pop()+stack.pop())".to_string()),
                    TokenType::TSub => loop_stack.last_mut().unwrap().push("stack.append(stack.pop()-stack.pop())".to_string()),
                    TokenType::TMul => loop_stack.last_mut().unwrap().push("stack.append(stack.pop()*stack.pop())".to_string()),
                    TokenType::TDiv => loop_stack.last_mut().unwrap().push("stack.append(stack.pop()/stack.pop())".to_string()),
                    TokenType::TDup => {
                        loop_stack.last_mut().unwrap().push("dup = stack.pop()".to_string());
                        loop_stack.last_mut().unwrap().push("stack.append(dup)".to_string());
                        loop_stack.last_mut().unwrap().push("stack.append(dup)".to_string());
                    },
                    TokenType::TFlip => {
                        loop_stack.last_mut().unwrap().push("assert len(stack) >= 2".to_string());
                        loop_stack.last_mut().unwrap().push("tmp = stack[-2]".to_string());
                        loop_stack.last_mut().unwrap().push("stack[-2] = stack[-1]".to_string());
                        loop_stack.last_mut().unwrap().push("stack[-1] = tmp".to_string());
                    },
                    TokenType::TPop => loop_stack.last_mut().unwrap().push("stack.pop()".to_string()),
                    TokenType::TEnd => {
                        let mut last = loop_stack.pop().unwrap();
                        last.push("if stack[-1] == 0:".to_string());
                        last.push("\tbreak".to_string());
                        let mut loop_code: Vec<String> = vec![];
                        for line in last {
                            loop_code.push(format!("\t{}", line));
                        }
                        if loop_stack.len() < 1 {
                            code.append(&mut loop_code);
                            state = State::Normal;
                        }
                        else {
                            loop_stack.last_mut().unwrap().append(&mut loop_code);
                        }
                    },
                    TokenType::TSemi => {
                        while stack.len() >= 1 {
                            loop_stack.last_mut().unwrap().push(format!("stack.append({})", stack.pop().unwrap()));
                        }
                    },
                }
            
            }
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
