use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::complete::{digit1, i64, one_of, u32},
    combinator::{eof, map, opt},
    multi::count,
    number::complete::double,
    sequence::terminated,
    IResult,
};

mod resp_type;
pub use resp_type::{Command, RespType};

pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RespType<'a>> {
    terminated(parse_chunk, eof)(input)
}
pub fn parse_chunk<'a>(input: &'a str) -> IResult<&'a str, RespType> {
    alt((
        parse_simple_string,
        parse_bulk_string,
        parse_bulk_error,
        parse_array,
        parse_simple_error,
        parse_bool,
        parse_int,
        parse_null,
        parse_double,
        parse_big_number,
        parse_v_string,
    ))(input)
}

///Parses slice into Simple String.
fn parse_simple_string(input: &str) -> IResult<&str, RespType> {
    let (input, _) = tag("+")(input)?;
    let (input, value) = parse_simple_string_raw(input)?;
    Ok((input, RespType::SString(value)))
}

///Parses slice into Simple Error.
///Simple Errors are Simple Strings but prefixed with a '-'.
///By convetion they include an error type, but the parser doesn't check for this type.
fn parse_simple_error(input: &str) -> IResult<&str, RespType> {
    let (input, _) = tag("-")(input)?;
    let (input, value) = parse_simple_string_raw(input)?;
    Ok((input, RespType::SError(value)))
}

///Simple string helper.
fn parse_simple_string_raw(input: &str) -> IResult<&str, &str> {
    terminated(take_while(|c| c != '\r' && c != '\n'), crlf)(input)
}

///CRLF tag helper
fn crlf(input: &str) -> IResult<&str, &str> {
    tag("\r\n")(input)
}

///Parses slice into an i64.
fn parse_int(input: &str) -> IResult<&str, RespType> {
    let (input, _) = tag(":")(input)?;
    let (input, value) = terminated(i64, crlf)(input)?;
    Ok((input, RespType::Int(value)))
}

///Bulk string helper.
fn parse_bulk_string_raw(input: &str) -> IResult<&str, &str> {
    let (input, len) = terminated(u32, crlf)(input)?;
    let (input, value) = terminated(take(len as usize), crlf)(input)?;
    Ok((input, value))
}

fn parse_bulk_string(input: &str) -> IResult<&str, RespType> {
    let (input, _) = tag("$")(input)?;
    let (input, value) = parse_bulk_string_raw(input)?;
    Ok((input, RespType::BString(value)))
}

fn parse_bulk_error(input: &str) -> IResult<&str, RespType> {
    let (input, _) = tag("!")(input)?;
    let (input, value) = parse_bulk_string_raw(input)?;
    Ok((input, RespType::BError(value)))
}

fn parse_array(input: &str) -> IResult<&str, RespType> {
    let (input, _) = tag("*")(input)?;
    let (input, len) = terminated(u32, crlf)(input)?;
    let (input, value) = count(parse_chunk, len as usize)(input)?;
    Ok((input, RespType::Array(value)))
}

// fn parse_command(input: &str) -> IResult<&str, RespType> {
//     let (_, command) = opt(alt((tag_no_case("ping"), tag_no_case("echo"))))(input)?;
//     Ok(input, command)
// }

fn parse_bool(input: &str) -> IResult<&str, RespType> {
    let (input, _) = tag("#")(input)?;
    let (input, value) = terminated(one_of("tf"), crlf)(input)?;
    let result = match value {
        't' => true,
        'f' => false,
        _ => unreachable!(),
    };
    Ok((input, RespType::Bool(result)))
}

fn parse_null(input: &str) -> IResult<&str, RespType> {
    let (input, _) = terminated(alt((tag("_"), tag("$-1"), tag("*-1"))), crlf)(input)?;
    Ok((input, RespType::Null))
}

fn parse_double(input: &str) -> IResult<&str, RespType> {
    let (input, _) = tag(",")(input)?;
    let (input, value) = terminated(
        alt((
            map(tag("+inf"), |_| f64::INFINITY),
            map(tag("-inf"), |_| f64::NEG_INFINITY),
            double,
        )),
        crlf,
    )(input)?;
    Ok((input, RespType::Double(value.to_string())))
}

fn parse_big_number(input: &str) -> IResult<&str, RespType> {
    let original = input;
    let (input, _) = tag("(")(input)?;
    let (input, sign) = opt(one_of("+-"))(input)?;
    let (input, value) = terminated(digit1, crlf)(input)?;
    let num_slice = &original[1..value.len() + if sign.is_some() { 2 } else { 1 }];
    Ok((input, RespType::BigNum(num_slice)))
}

fn parse_v_string(input: &str) -> IResult<&str, RespType> {
    let (input, _) = tag("=")(input)?;
    let (input, len) = terminated(u32, crlf)(input)?;
    let (input, encoding) = terminated(take(3 as usize), tag(":"))(input)?;
    let (input, value) = terminated(take(len - 4), crlf)(input)?;
    Ok((input, RespType::VString(encoding, value)))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_simple_string_test() {
        let input = "+Ok\r\n";
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
        println!("{}", parse(input).is_err());
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_double_test() {
        let input = ",1.23\r\n";
        assert_eq!(
            parse(input).unwrap().1,
            RespType::Double("1.23".to_string())
        );
        let input = ",10\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::Double(10.to_string()));
        let input = ",inf\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::Double("inf".to_string()));
        let input = ",-inf\r\n";
        assert_eq!(
            parse(input).unwrap().1,
            RespType::Double("-inf".to_string())
        );
        let input = ",nan\r\n";
        assert_eq!(parse(input).unwrap().1, RespType::Double("NaN".to_string()));
    }

    #[test]
    fn parse_big_number_test() {
        let input = "(3492890328409238509324850943850943825024385\r\n";
        assert_eq!(
            parse(input),
            Ok((
                "",
                RespType::BigNum("3492890328409238509324850943850943825024385")
            ))
        );

        let input = "(-3492890328409238509324850943850943825024385\r\n";
        assert_eq!(
            parse(input),
            Ok((
                "",
                RespType::BigNum("-3492890328409238509324850943850943825024385")
            ))
        );

        let input = "(-3492890asd328409238509324850943850943825024385\r\n";
        assert!(parse(input).is_err());
    }

    #[test]
    fn parse_v_string_test() {
        let input = "=15\r\ntxt:Some string\r\n";
        assert_eq!(
            parse(input),
            Ok(("", RespType::VString("txt", "Some string")))
        );
    }
}
