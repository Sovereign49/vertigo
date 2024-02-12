use crate::tokenizer::*;
use vertigo::{OpCode, StackItem};

pub fn tokens_to_vm(tokens: Vec<Token>) -> Vec<u8> {
    let mut code: Vec<u8> = vec![];
    code.push(0x69);
    let mut stack: Vec<StackItem> = vec![];
    for token in tokens {
        match token.ttype {
            TokenType::TStrLit => stack.push(StackItem::TStr(token.value.unwrap())),
            TokenType::TNumLit => {
                stack.push(StackItem::TNum(token.value.unwrap().parse().unwrap()))
            }
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
                        }
                        StackItem::TStr(x) => {
                            code.push(OpCode::OpPush as u8);
                            code.push(0x02);
                            let bytes = x.as_bytes();
                            code.push(bytes.len() as u8);
                            for byte in bytes {
                                code.push(*byte);
                            }
                        }
                    }
                }
            }
            TokenType::TPrint => code.push(OpCode::OpPrint as u8),
            TokenType::TAdd => code.push(OpCode::OpAdd as u8),
            TokenType::TSub => code.push(OpCode::OpSub as u8),
            TokenType::TMul => code.push(OpCode::OpMul as u8),
            TokenType::TDiv => code.push(OpCode::OpDiv as u8),
            TokenType::TLoop => code.push(OpCode::OpLoop as u8),
            TokenType::TEnd => code.push(OpCode::OpEnd as u8),
            TokenType::TDup => code.push(OpCode::OpDup as u8),
            TokenType::TFlip => code.push(OpCode::OpFlip as u8),
            TokenType::TPop => code.push(OpCode::OpPop as u8),
            _ => todo!("Not implemented"),
        }
    }
    code.push(OpCode::OpExit as u8);
    code.push(0x00);

    return code;
}
