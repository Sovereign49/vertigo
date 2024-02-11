use std::env;
use std::fs::File;
use std::io::prelude::*;
use vertigo::{OpCode, StackItem};

fn main() {
    if env::args().len() < 2 {
        println!("Incorrect command usage");
        println!("Propper usage is as follows: ");
        println!("vertigo <program>");
        std::process::exit(1);
    }
    // read file as bytes
    let argv: Vec<String> = env::args().collect();
    let mut src_file = File::open(&argv[1]).expect("Failed to open source file");
    let mut bytes: Vec<u8> = vec![];
    let _ = src_file.read_to_end(&mut bytes);
    if bytes[0] != 0x69 {
        println!("Failure to read file: Invalid File Format");
        std::process::exit(1);
    }
    let mut stack: Vec<StackItem> = vec![];
    let mut i = 1;
    
    while i < bytes.len() {
        let operation = OpCode::try_from(bytes[i]).unwrap();
        match operation {
            OpCode::OpPush => {
                i+=1;
                match bytes[i] {
                    0x00 => (),
                    0x01 => {
                        let mut f32_arr: [u8;4] = [0,0,0,0];
                        for j in 0..4 {
                            i+=1;
                            f32_arr[j] = bytes[i];
                        } 
                        stack.push(StackItem::TNum(f32::from_le_bytes(f32_arr)));
                    },
                    0x02 => {
                        i+=1;
                        let len = bytes[i];
                        let mut str_arr: Vec<u8> = vec![];
                        for _j in 0..len {
                            i+=1;
                            str_arr.push(bytes[i]);
                        } 
                        stack.push(StackItem::TStr(String::from_utf8(str_arr).unwrap()));
                    },
                    _ => {println!("Error Invalid type");std::process::exit(1);}
                }
            },
            OpCode::OpPrint => {
                let item = stack.pop().unwrap();
                if let StackItem::TStr(s) = item {
                    println!("{}", s);    
                }
                else if let StackItem::TNum(s) = item {
                    println!("{}", s);    
                }
            },
            OpCode::OpExit => { i+=1;std::process::exit(bytes[i].into());},
            OpCode::OperationCount => assert!(false, "You shouldn't be here"),
            OpCode::OpAdd => { 
                let item1 = stack.pop().unwrap();
                let item2 = stack.pop().unwrap();
                if let StackItem::TNum(n1) = item1 {
                    if let StackItem::TNum(n2) = item2 {
                        stack.push(StackItem::TNum(n1 + n2));
                    }
                    else {
                        panic!("error adding a string");
                    }
                }
                else {
                    panic!("error adding a string");
                }
            },
            OpCode::OpSub => { 
                let item1 = stack.pop().unwrap();
                let item2 = stack.pop().unwrap();
                if let StackItem::TNum(n1) = item1 {
                    if let StackItem::TNum(n2) = item2 {
                        stack.push(StackItem::TNum(n1 - n2));
                    }
                    else {
                        panic!("error adding a string");
                    }
                }
                else {
                    panic!("error adding a string");
                }
            },
            OpCode::OpMul => { 
                let item1 = stack.pop().unwrap();
                let item2 = stack.pop().unwrap();
                if let StackItem::TNum(n1) = item1 {
                    if let StackItem::TNum(n2) = item2 {
                        stack.push(StackItem::TNum(n1 * n2));
                    }
                    else {
                        panic!("error adding a string");
                    }
                }
                else {
                    panic!("error adding a string");
                }
            },
            OpCode::OpDiv => { 
                let item1 = stack.pop().unwrap();
                let item2 = stack.pop().unwrap();
                if let StackItem::TNum(n1) = item1 {
                    if let StackItem::TNum(n2) = item2 {
                        stack.push(StackItem::TNum(n1 / n2));
                    }
                    else {
                        panic!("error adding a string");
                    }
                }
                else {
                    panic!("error adding a string");
                }
            },
        }
        i+=1;
    }
}
