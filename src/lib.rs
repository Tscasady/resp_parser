use nom::{character::complete::one_of, IResult};

//TODO: Map / sets might have to be separate.
#[derive(Debug)]
pub enum RespType<'a> {
    SString(&'a str),
    BString,
    SError,
    BError,
    Array,
    Null,
    Bool,
    Double,
    BigNum,
    Int,
    VString,
    Map,
    Set,
    Push,
}

pub fn parse(input: &str) -> IResult<&str, RespType> {
    let (input, resp_type) = one_of("+*-:$_#,(=%!~>")(input)?;
    match resp_type {
        '+' => parse_simple_string(input),
        '*' => {
            todo!()
        }
        '-' => {
            todo!()
        }
        ':' => {
            todo!()
        }
        '$' => {
            todo!()
        }
        '_' => {
            todo!()
        }
        '#' => {
            todo!()
        }
        ',' => {
            todo!()
        }
        '(' => {
            todo!()
        }
        '=' => {
            todo!()
        }
        '%' => {
            todo!()
        }
        '!' => {
            todo!()
        }
        '~' => {
            todo!()
        }
        '>' => {
            todo!()
        }
        '}' => {
            todo!()
        }
        _ => unreachable!(),
    }
}

fn parse_simple_string(input: &str) -> IResult<&str, RespType> {
    let (input, value) = todo!();
    Ok((input, RespType::SString(value)))
}
