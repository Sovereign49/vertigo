pub enum StackItem {
    TStr(String),
    TNum(f32),
}

#[allow(dead_code)]
pub enum OpCode {
    OpPush,
    OpPrint,
    OpExit,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OperationCount
}

impl std::convert::TryFrom<u8> for OpCode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x >= OpCode::OpPush as u8 && x < OpCode::OperationCount as u8 =>
                Ok(unsafe { std::mem::transmute(x) }),
            _ => Err(()),
        }
    }
}
