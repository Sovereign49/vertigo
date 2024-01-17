pub enum TokenType {
    TStrLit=0,
    TPrint=1,
    TSemi=2,
}

pub struct Token {
    pub ttype: TokenType,
    pub value: Option<String>
}

fn isalnum(character: u8) -> bool {
    if 48<=character && character <=57||65<=character && character <=90||97<=character && character <=122 {
        return true;
    }
    false
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

pub fn tokenize(src: &str) -> Vec<Token>{
    let mut tokens: Vec<Token> = vec![];
    let mut src_arr: Vec<Vec<u8>> = src.lines().map(|x| x.as_bytes().to_vec()).collect();
    let mut buf: Vec<u8> = vec![];
    let len = get_len(&src_arr);

    for i in 0..src_arr.len() {
        if src_arr[i].len() < len {
            let mut spaces: Vec<u8> = vec![]; 
            for _i in 0..(len-src_arr[i].len()){
                spaces.push(b' ');
            }
            src_arr[i].append(&mut spaces);
        }
    }
    for i in 0..len {
        let mut j = 0;
        while j < src_arr.len() {
            let mut c: u8 = src_arr[j][i];
            if isalnum(c) {
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
                }
                continue;
            }
            if c == b'"'{
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
