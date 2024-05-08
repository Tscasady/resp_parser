use nom::{
    bytes::complete::{tag, take, take_while}, character::complete::{i32, i64, one_of}, combinator::eof, sequence::terminated, Err, IResult
};

//TODO: Map / sets might have to be separate.
#[derive(Debug, PartialEq, Eq)]
pub enum RespType<'a> {
    SString(&'a str),
    BString(&'a str),
    SError(&'a str),
    BError,
    Array(Vec<RespType<'a>>),
    Null,
    Bool(bool),
    Double,
    BigNum,
    Int(i64),
    VString,
    Map,
    Set,
    Push,
}

pub fn parse<'a>(input: &'a str) -> IResult<&str, RespType<'a>> {
    terminated(parse_chunk, eof)(input)
}
pub fn parse_chunk(input: &str) -> IResult<&str, RespType> {
    let (input, resp_type) = one_of("+*-:$_#,(=%!~>")(input)?;
    match resp_type {
        '+' => parse_simple_string(input),
        '*' => parse_array(input),
        '-' => parse_simple_error(input),
        ':' => parse_int(input),
        '$' => parse_bulk_string(input),
        '_' => parse_null(input),
        '#' => parse_bool(input),
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
        return Ok((input, RespType::Null));
    } else {
        let (input, value) = terminated(take(len as usize), crlf)(input)?;
        Ok((input, RespType::BString(value)))
    }
}

fn parse_array(input: &str) -> IResult<&str, RespType> {
    let (mut input, len) = terminated(i32, crlf)(input)?;
    if len == -1 {
        return Ok((input, RespType::Null));
    } else if len < -1 {
        return Ok((
            input,
            RespType::SError("Tried to parse negative length array."),
        ));
    } else {
        let mut result = Vec::new();
        let mut value;
        for _i in 0..len {
            (input, value) = parse_chunk(input)?;
            result.push(value)
        }
        Ok((input, RespType::Array(result)))
    }
}

fn parse_bool(input: &str) -> IResult<&str, RespType> {
    let (input, value) = terminated(one_of("tf"), crlf)(input)?;
    let result = match value {
        't' => true,
        'f' => false,
        _ => unreachable!(),
    };
    Ok((input, RespType::Bool(result)))
}

fn parse_null(input: &str) -> IResult<&str, RespType> {
    let (input, value) = crlf(input)?;
    if !input.is_empty() {
        return Err(Err::Error(nom::error::Error {code: nom::error::ErrorKind::NonEmpty, input: "had stuff"}))
    } else {
        Ok((input, RespType::Null))
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
    fn parse_bulk_string_test() {
        let input = "$5\r\nhello\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::BString("hello"));
        let input = "$10\r\nhello\r\nfoo\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::BString("hello\r\nfoo"));
        let input = "$-1\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::Null);
        let input = "$0\r\n\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::BString(""));
    }

    #[test]
    fn parse_array_test() {
        let input = "*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n";
        assert_eq!(
            parse(input).unwrap().1,
            RespType::Array(vec![RespType::BString("hello"), RespType::BString("world")])
        );

        let input = "*0\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::Array(vec![]));

        let input = "*-1\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::Null);
    }

    #[test]
    fn parse_bool_test() {
        let input = "#t\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::Bool(true));
    }

    #[test]
    fn parse_null() {
        let input = "_\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::Null);

        let input = "_stuff\r\n";
        assert!(parse(input).is_err());

    }
}
