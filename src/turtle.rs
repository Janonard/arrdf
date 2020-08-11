use nom::branch::*;
use nom::bytes::streaming::*;
use nom::character::streaming::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::*;
use std::convert::TryFrom;

#[derive(Debug)]
enum FromHexError {
    ToCharError,
    ToU32Error,
}

fn from_hex(input: &str) -> Result<char, FromHexError> {
    char::try_from(u32::from_str_radix(input, 16).map_err(|_| FromHexError::ToU32Error)?)
        .map_err(|_| FromHexError::ToCharError)
}

fn u16_char(i: &str) -> IResult<&str, char> {
    let (i, (_, _, c)) = tuple((
        char('\\'),
        char('u'),
        map_res(take_while_m_n(4, 4, |c: char| c.is_digit(16)), from_hex),
    ))(i)?;
    Ok((i, c))
}

fn u32_char(i: &str) -> IResult<&str, char> {
    let (i, (_, _, c)) = tuple((
        char('\\'),
        char('U'),
        map_res(take_while_m_n(8, 8, |c: char| c.is_digit(16)), from_hex),
    ))(i)?;
    Ok((i, c))
}

fn iriref(i: &str) -> IResult<&str, String> {
    let (i, iri) = delimited(
        char('<'),
        many0(alt((none_of(r#"<>"{}|^`|\"#), u16_char, u32_char))),
        char('>'),
    )(i)?;

    Ok((i, iri.into_iter().collect()))
}

#[test]
fn test_iriref() {
    assert_eq!((" abc", String::from("urn:Übung")), iriref(r"<urn:\u00DCbung> abc").unwrap());
    assert_eq!((" abc", String::from("urn:Übung")), iriref(r"<urn:\U000000DCbung> abc").unwrap());
}
