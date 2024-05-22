use std::collections::BTreeMap;
// use crate::command::{Command, Commands};

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

impl<'a> RespType<'a> {
    pub fn inner(&self) -> &'a str {
        match self {
            RespType::BString(value) => value,
            RespType::SString(value) => value,
            _ => unimplemented!(),
        }
    }
}
