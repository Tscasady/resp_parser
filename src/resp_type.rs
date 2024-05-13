use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
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
    Echo {
        args: Vec<RespType<'a>>,
    },
    Set {
        key: String,
        value: String,
        px: Option<u64>,
    },
    Get {
        args: Vec<RespType<'a>>,
    },
}

impl<'a> RespType<'a> {
    pub fn to_command(self) -> Result<Command<'a>, String> {
        match self {
            RespType::Array(mut args) => match args[0] {
                RespType::BString(command) => {
                    match command.to_lowercase().as_str() {
                        "ping" => Ok(Command::Ping),
                        "set" => { let px = args.get(4).map(|resptype| resptype.inner().parse::<u64>().expect("Set command px value should be able to be parsed to u64."));
                            Ok(Command::Set { key: args[1].inner().to_string(), value: args[2].inner().to_string(), px})},
                        "get" => Ok(Command::Get { args: args.drain(1..).collect() }),
                        "echo" => Ok(Command::Echo { args: args.drain(1..).collect()}),
                        _ => Err(format!("Not a known command: {}", command))
                    }
                }
                _ => Err(format!("First element of Array must be a bulk String in order to be a command. Given {:?}", args[0]))
            },
            _ => Err(format!("Only RespType::Array can be converted to commands."))
        }
    }

    pub fn inner(&self) -> &'a str {
        match self {
            RespType::BString(value) => value,
            RespType::SString(value) => value,
            _ => unimplemented!(),
        }
    }
}
