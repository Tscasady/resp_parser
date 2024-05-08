use nom::{
    bytes::complete::{tag, take, take_while},
    character::complete::{i32, i64, one_of},
    combinator::eof,
    sequence::terminated,
    IResult,
};

//TODO: Map / sets might have to be separate.
#[derive(Debug, PartialEq, Eq)]
pub enum RespType<'a> {
    SString(&'a str),
    BString(&'a str),
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
    let (input, len) = terminated(i32, crlf)(input)?;
    if len == -1 {
        return Ok((input, RespType::Null))
    } else {
        let (input, value) = terminated(take(len as usize), crlf)(input)?;
        Ok((input, RespType::BString(value)))
    }
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

    #[test]
    fn parse_bulk_string() {
        let input = "$5\r\nhello\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::BString("hello"));
        let input = "$-1\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::Null);
        let input = "$0\r\n\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::BString(""));
    }

}
