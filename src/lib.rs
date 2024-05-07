use nom::{bytes::complete::{tag, take_until, take_while}, character::complete::one_of, sequence::terminated, IResult};

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
            parse_bulk_string(input)
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

///Parses slice into Simple String.
fn parse_simple_string(input: &str) -> IResult<&str, RespType> {
    let (input, value) = terminated(take_while(|c| c != '\r' && c != '\n'), crlf)(input)?;
    Ok((input, RespType::SString(value)))
}

///CRLF tag helper
fn crlf(input: &str) -> IResult<&str, &str> {
    tag("\r\n")(input)
}

fn parse_bulk_string(input: &str) -> IResult<&str, RespType> {
    //cant contain clrf before end
    //must end wit clrf
    //must be correct len?
    // Ok((input, RespType::BString(value)))
    todo!()

}

