use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

enum TokenType {
    TStrLit=0,
    TPrint=1,
    TSemi=2,
}

struct Token {
    ttype: TokenType,
    value: Option<String>
}

fn isalnum(character: u8) -> bool {
    if 48<=character && character <=57||65<=character && character <=90||97<=character && character <=122 {
        return true;
    }
    false
}

fn tokenize(src: &str) -> Vec<Token>{
    let mut tokens: Vec<Token> = vec![];
    let src_arr: Vec<_> = src.lines().collect();
    let mut buf: Vec<u8> = vec![];
    let long = src_arr.iter().fold(src_arr[0], |acc, &item| {
        if item.len() > acc.len() {
            item
        } else {
            acc
        }
    });
    for i in 0..long.len() {
        let mut j = 0;
        while j < src_arr.len() {
            let mut c: u8 = src_arr[j].as_bytes()[i];
            if isalnum(c) {
                while isalnum(c) {
                    buf.push(c);
                    j += 1;
                    c = src_arr[j].as_bytes()[i];
                }   
                if buf == b"print" {
                tokens.push(Token {
                    ttype: TokenType::TPrint,
                    value: None,
                });
                }
                continue;
            }
            if c == b'"'{
                j += 1;
                c = src_arr[j].as_bytes()[i];
                loop {
                    buf.push(c);
                    j += 1;
                    c = src_arr[j].as_bytes()[i];
                    if c == b'"' {
                        j += 1;
                        break;
                    }
                }   
                let string_value = String::from_utf8(buf.clone()).unwrap();
                tokens.push(Token {
                    ttype: TokenType::TStrLit,
                    value: Some(string_value)
                });
                buf.clear();
                continue;
            }
            if c == b';' {
                tokens.push(Token {
                    ttype: TokenType::TSemi,
                    value: None
                });
                break;
            }
            j += 1;
        } 
    }
    return tokens;
}

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
