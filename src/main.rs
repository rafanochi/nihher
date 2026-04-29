use nom::Parser;
use nom::combinator::value;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::IResult;
use std::error::Error;
use nom::character::complete::alpha0;

fn parse_let(input: &str) -> IResult<&str, &str> {
    alt((
        tag("let"),
    )).parse(input)
}

pub fn parse_letters(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

pub fn parse_bool(input: &str) -> IResult<&str, bool> {
    alt((
            value(true, tag("true")),
            value(false, tag("false")),
    )).parse(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    // letter parser
    let (remaining_input, letters) = parse_letters("abclet@#$@#4123")?;
    assert_eq!(remaining_input, "@#$@#4123");
    assert_eq!(letters, "abclet");


    // let or def 
    let (remaining_input, out) = parse_let("letdef")?;
    assert_eq!(out, "let");


    // boolean 
    let (remaining_input, boolean) = parse_bool("true124")?;
    assert_eq!(boolean, true);

    Ok(())
}
