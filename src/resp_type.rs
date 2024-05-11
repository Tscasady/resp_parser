use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RespType<'a> {
    SString(&'a str),
    BString(&'a str),
    SError(&'a str),
    BError(&'a str),
    Array(Vec<RespType<'a>>),
    Null,
    Bool(bool),
    Double(String),
    BigNum(&'a str),
    Int(i64),
    VString(&'a str, &'a str),
    Map(BTreeMap<RespType<'a>, RespType<'a>>),
    Set,
    Push,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Command<'a> {
    Ping,
    Echo { args: Vec<RespType<'a>> },
}

impl<'a> RespType<'a> {
    pub fn to_command(self) -> Result<Command<'a>, String> {
        match self {
            RespType::Array(args) => match args[0] {
                RespType::BString(command) => {
                    match command.to_lowercase().as_str() {
                        "ping" => Ok(Command::Ping),
                        "echo" => Ok(Command::Echo { args: args[1..].to_vec()}),
                        _ => Err(format!("Not a known command: {}", command))
                    }
                }
                _ => Err("Cannot be converted to command.".to_string())
            },
            _ => Err("Only RespType::Array can be converted to commands.".to_string())
        }
    }
}
