use crate::tokenizer::*;
use vertigo::{OpCode, StackItem};

pub fn tokens_to_py(tokens: Vec<Token>) -> Vec<u8> {
    enum State {
        Normal,
        Looping,
    }
    let mut state = State::Normal;
    let mut code: Vec<String> = vec!["stack = []".to_string()];
    let mut stack: Vec<String> = vec![];
    let mut loop_stack: Vec<Vec<String>> = vec![];
    for token in tokens {
        match state {
            State::Normal => match token.ttype {
                TokenType::TStrLit => stack.push(format!("\"{}\"", token.value.unwrap())),
                TokenType::TNumLit => stack.push(format!("{}", token.value.unwrap())),
                TokenType::TPrint => code.push("print(stack.pop())".to_string()),
                TokenType::TAsAscii => {
                    code.push("pop_str = stack.pop()".to_string());
                    code.push("for i in range(len(pop_str)):".to_string());
                    code.push("\tstack.append(ord(pop_str[i]))".to_string());
                }
                TokenType::TLoop => {
                    code.push("while True:".to_string());
                    loop_stack.push(vec![]);
                    state = State::Looping;
                }
                TokenType::TAdd => code.push("stack.append(stack.pop()+stack.pop())".to_string()),
                TokenType::TSub => code.push("stack.append(stack.pop()-stack.pop())".to_string()),
                TokenType::TMul => code.push("stack.append(stack.pop()*stack.pop())".to_string()),
                TokenType::TDiv => code.push("stack.append(stack.pop()/stack.pop())".to_string()),
                TokenType::TDup => {
                    code.push("dup = stack.pop()".to_string());
                    code.push("stack.append(dup)".to_string());
                    code.push("stack.append(dup)".to_string());
                }
                TokenType::TFlip => {
                    code.push("assert len(stack) >= 2".to_string());
                    code.push("tmp = stack[-2]".to_string());
                    code.push("stack[-2] = stack[-1]".to_string());
                    code.push("stack[-1] = tmp".to_string());
                }
                TokenType::TPop => code.push("stack.pop()".to_string()),
                TokenType::TEnd => {
                    println!("Error ending outside of loop");
                    std::process::exit(1);
                }
                TokenType::TSemi => {
                    while stack.len() >= 1 {
                        code.push(format!("stack.append({})", stack.pop().unwrap()));
                    }
                }
            },
            State::Looping => match token.ttype {
                TokenType::TStrLit => stack.push(format!("\"{}\"", token.value.unwrap())),
                TokenType::TNumLit => stack.push(format!("{}", token.value.unwrap())),
                TokenType::TPrint => loop_stack
                    .last_mut()
                    .unwrap()
                    .push("print(stack.pop())".to_string()),
                TokenType::TAsAscii => {
                    loop_stack
                        .last_mut()
                        .unwrap()
                        .push("pop_str = stack.pop()".to_string());
                    loop_stack
                        .last_mut()
                        .unwrap()
                        .push("for i in range(len(pop_str)):".to_string());
                    loop_stack
                        .last_mut()
                        .unwrap()
                        .push("\tstack.append(ord(pop_str[i]))".to_string());
                }
                TokenType::TLoop => {
                    loop_stack
                        .last_mut()
                        .unwrap()
                        .push("while True:".to_string());
                    loop_stack.push(vec![]);
                    state = State::Looping;
                }
                TokenType::TAdd => loop_stack
                    .last_mut()
                    .unwrap()
                    .push("stack.append(stack.pop()+stack.pop())".to_string()),
                TokenType::TSub => loop_stack
                    .last_mut()
                    .unwrap()
                    .push("stack.append(stack.pop()-stack.pop())".to_string()),
                TokenType::TMul => loop_stack
                    .last_mut()
                    .unwrap()
                    .push("stack.append(stack.pop()*stack.pop())".to_string()),
                TokenType::TDiv => loop_stack
                    .last_mut()
                    .unwrap()
                    .push("stack.append(stack.pop()/stack.pop())".to_string()),
                TokenType::TDup => {
                    loop_stack
                        .last_mut()
                        .unwrap()
                        .push("dup = stack.pop()".to_string());
                    loop_stack
                        .last_mut()
                        .unwrap()
                        .push("stack.append(dup)".to_string());
                    loop_stack
                        .last_mut()
                        .unwrap()
                        .push("stack.append(dup)".to_string());
                }
                TokenType::TFlip => {
                    loop_stack
                        .last_mut()
                        .unwrap()
                        .push("assert len(stack) >= 2".to_string());
                    loop_stack
                        .last_mut()
                        .unwrap()
                        .push("tmp = stack[-2]".to_string());
                    loop_stack
                        .last_mut()
                        .unwrap()
                        .push("stack[-2] = stack[-1]".to_string());
                    loop_stack
                        .last_mut()
                        .unwrap()
                        .push("stack[-1] = tmp".to_string());
                }
                TokenType::TPop => loop_stack
                    .last_mut()
                    .unwrap()
                    .push("stack.pop()".to_string()),
                TokenType::TEnd => {
                    let mut last = loop_stack.pop().unwrap();
                    last.push("if stack[-1] == 0:".to_string());
                    last.push("\tstack.pop()".to_string());
                    last.push("\tbreak".to_string());
                    let mut loop_code: Vec<String> = vec![];
                    for line in last {
                        loop_code.push(format!("\t{}", line));
                    }
                    if loop_stack.len() < 1 {
                        code.append(&mut loop_code);
                        state = State::Normal;
                    } else {
                        loop_stack.last_mut().unwrap().append(&mut loop_code);
                    }
                }
                TokenType::TSemi => {
                    while stack.len() >= 1 {
                        loop_stack
                            .last_mut()
                            .unwrap()
                            .push(format!("stack.append({})", stack.pop().unwrap()));
                    }
                }
            },
        }
    }
    code.join("\n").as_bytes().to_vec()
}

pub fn tokens_to_vm(tokens: Vec<Token>) -> Vec<u8> {
    let mut code: Vec<u8> = vec![];
    code.push(0x69);
    let mut stack: Vec<StackItem> = vec![];
    for token in tokens {
        match token.ttype {
            TokenType::TStrLit => stack.push(StackItem::TStr(token.value.unwrap())),
            TokenType::TNumLit => stack.push(StackItem::TNum(token.value.unwrap().parse().unwrap())),
            TokenType::TSemi => {
                while stack.len() > 0 {
                    let item = stack.pop().unwrap();
                    match item {
                        StackItem::TNum(x) => {
                            code.push(OpCode::OpPush as u8);
                            code.push(0x01);
                            let bytes = x.to_le_bytes();
                            for byte in bytes {
                                code.push(byte);
                            }
                        },
                        StackItem::TStr(x) => {
                            code.push(OpCode::OpPush as u8);
                            code.push(0x02);
                            let bytes = x.as_bytes();
                            code.push(bytes.len() as u8);
                            for byte in bytes {
                                code.push(*byte);
                            }
                        },
                    }
                }
            },
            TokenType::TPrint => code.push(OpCode::OpPrint as u8),
            TokenType::TAdd => code.push(OpCode::OpPrint as u8),
            _ => todo!("Not implemented")
        }
    }
    code.push(OpCode::OpExit as u8);
    code.push(0x00);

    return code;
}
