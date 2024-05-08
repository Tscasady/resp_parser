use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{i64, one_of},
    combinator::eof,
    sequence::terminated,
    IResult,
};

//TODO: Map / sets might have to be separate.
#[derive(Debug, PartialEq, Eq)]
pub enum RespType<'a> {
    SString(&'a str),
    BString,
    SError(&'a str),
    BError,
    Array,
    Null,
    Bool,
    Double,
    BigNum,
    Int(i64),
    VString,
    Map,
    Set,
    Push,
}

pub fn parse(input: &str) -> IResult<&str, RespType> {
    let (input, resp_type) = one_of("+*-:$_#,(=%!~>")(input)?;
    let parser = match resp_type {
        '+' => parse_simple_string,
        '*' => todo!(),
        '-' => parse_simple_error,
        ':' => parse_int,
        '$' => parse_bulk_string,
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
    };
    terminated(parser, eof)(input)
}

///Parses slice into Simple String.
fn parse_simple_string(input: &str) -> IResult<&str, RespType> {
    let (input, value) = parse_simple_string_raw(input)?;
    Ok((input, RespType::SString(value)))
}

///Parses slice into Simple Error.
///Simple Errors are Simple Strings but prefixed with a '-'.
///By convetion they include an error type, but the parser doesn't check for this type.
fn parse_simple_error(input: &str) -> IResult<&str, RespType> {
    let (input, value) = parse_simple_string_raw(input)?;
    Ok((input, RespType::SError(value)))
}

fn parse_simple_string_raw(input: &str) -> IResult<&str, &str> {
    terminated(take_while(|c| c != '\r' && c != '\n'), crlf)(input)
}

///CRLF tag helper
fn crlf(input: &str) -> IResult<&str, &str> {
    tag("\r\n")(input)
}

fn parse_int(input: &str) -> IResult<&str, RespType> {
    let (input, value) = terminated(i64, crlf)(input)?;
    Ok((input, RespType::Int(value)))
}

fn parse_bulk_string(input: &str) -> IResult<&str, RespType> {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_simple_string_test() {
        let input = "Ok\r\n";
        assert_eq!(
            parse_simple_string(input).unwrap().1,
            RespType::SString("Ok")
        );
    }

    #[test]
    fn parse_i64_test() {
        let input = ":-21\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::Int(-21));
    }
}
