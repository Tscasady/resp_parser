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
                _ => Err(format!("First element of Array must be a bulk String in order to be a command. Given {:?}", args[0]))
            },
            _ => Err("Only RespType::Array can be converted to commands.".to_string())
        }
    }

    pub fn inner(&self) -> &'a str {
        match self {
            RespType::BString(value) => value,
            _ => unimplemented!(),
        }
    }
}
