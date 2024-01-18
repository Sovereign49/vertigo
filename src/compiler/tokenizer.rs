pub enum TokenType {
    TStrLit,
    TNumLit,
    TAsAscii,
    TPrint,
    TLoop,
    TEnd,
    TSemi,
    TAdd,
    TSub,
    TMul,
    TDiv,
    TDup,
    TFlip,
    TPop,
}

pub struct Token {
    pub ttype: TokenType,
    pub value: Option<String>,
}

fn isalnum(character: u8) -> bool {
    return 48 <= character && character <= 57
        || 65 <= character && character <= 90
        || 97 <= character && character <= 122;
}

fn isalpha(character: u8) -> bool {
    return 65 <= character && character <= 90 || 97 <= character && character <= 122;
}

fn isnumeric(character: u8) -> bool {
    return 48 <= character && character <= 57 || character == 46;
}

fn get_len<T>(arr: &Vec<Vec<T>>) -> usize {
    let mut len = 0;
    for line in arr {
        if line.len() > len {
            len = line.len();
        }
    }
    return len;
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut src_arr: Vec<Vec<u8>> = src.lines().map(|x| x.as_bytes().to_vec()).collect();
    let mut buf: Vec<u8> = vec![];
    let len = get_len(&src_arr);

    for i in 0..src_arr.len() {
        if src_arr[i].len() < len {
            let mut spaces: Vec<u8> = vec![];
            for _i in 0..(len - src_arr[i].len()) {
                spaces.push(b' ');
            }
            src_arr[i].append(&mut spaces);
        }
    }
    for i in 0..len {
        let mut j = 0;
        while j < src_arr.len() {
            let mut c: u8 = src_arr[j][i];
            if isalpha(c) {
                while isalnum(c) {
                    buf.push(c);
                    j += 1;
                    c = src_arr[j][i];
                }
                if buf == b"print" {
                    tokens.push(Token {
                        ttype: TokenType::TPrint,
                        value: None,
                    });
                } else if buf == b"loop" {
                    tokens.push(Token {
                        ttype: TokenType::TLoop,
                        value: None,
                    });
                } else if buf == b"end" {
                    tokens.push(Token {
                        ttype: TokenType::TEnd,
                        value: None,
                    });
                } else if buf == b"end" {
                    tokens.push(Token {
                        ttype: TokenType::TEnd,
                        value: None,
                    });
                } else if buf == b"ascii" {
                    tokens.push(Token {
                        ttype: TokenType::TAsAscii,
                        value: None,
                    });
                } else if buf == b"dup" {
                    tokens.push(Token {
                        ttype: TokenType::TDup,
                        value: None,
                    });
                } else if buf == b"flip" {
                    tokens.push(Token {
                        ttype: TokenType::TFlip,
                        value: None,
                    });
                } else if buf == b"pop" {
                    tokens.push(Token {
                        ttype: TokenType::TPop,
                        value: None,
                    });
                } else {
                    println!("invalid keyword");
                    std::process::exit(1);
                }
                buf.clear();
                continue;
            }
            if isnumeric(c) {
                while isnumeric(c) {
                    buf.push(c);
                    j += 1;
                    c = src_arr[j][i];
                }
                let string_value = String::from_utf8(buf.clone()).unwrap();
                tokens.push(Token {
                    ttype: TokenType::TNumLit,
                    value: Some(string_value),
                });
                buf.clear();
                continue;
            }
            if c == b'"' {
                j += 1;
                c = src_arr[j][i];
                while c != b'"' {
                    buf.push(c);
                    j += 1;
                    c = src_arr[j][i];
                }
                j += 1;
                let string_value = String::from_utf8(buf.clone()).unwrap();
                tokens.push(Token {
                    ttype: TokenType::TStrLit,
                    value: Some(string_value),
                });
                buf.clear();
                continue;
            }
            if c == b';' {
                tokens.push(Token {
                    ttype: TokenType::TSemi,
                    value: None,
                });
                break;
            }
            if c == b'+' {
                tokens.push(Token {
                    ttype: TokenType::TAdd,
                    value: None,
                });
                break;
            }
            if c == b'-' {
                tokens.push(Token {
                    ttype: TokenType::TSub,
                    value: None,
                });
                break;
            }
            if c == b'*' {
                tokens.push(Token {
                    ttype: TokenType::TMul,
                    value: None,
                });
                break;
            }
            if c == b'/' {
                tokens.push(Token {
                    ttype: TokenType::TDiv,
                    value: None,
                });
                break;
            }
            j += 1;
        }
    }
    return tokens;
}
