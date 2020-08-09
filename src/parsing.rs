use nom::character::streaming::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;
use std::convert::TryFrom;
use nom::branch::*;

fn hex_char(i: &str) -> IResult<&str, u32> {
    let (i, c) = one_of("0123456789abcdefABCDEF")(i)?;
    match c {
        '0' => Ok((i, 0)),
        '1' => Ok((i, 1)),
        '2' => Ok((i, 2)),
        '3' => Ok((i, 3)),
        '4' => Ok((i, 4)),
        '5' => Ok((i, 5)),
        '6' => Ok((i, 6)),
        '7' => Ok((i, 7)),
        '8' => Ok((i, 8)),
        '9' => Ok((i, 9)),
        'a' | 'A' => Ok((i, 10)),
        'b' | 'B' => Ok((i, 11)),
        'c' | 'C' => Ok((i, 12)),
        'd' | 'D' => Ok((i, 13)),
        'e' | 'E' => Ok((i, 14)),
        'f' | 'F' => Ok((i, 15)),
        _ => panic!("Invalid character matched by nom::character::streaming_::one_of"),
    }
}

#[test]
fn test_hex_char() {
    assert_eq!(("abc", 15), hex_char("fabc").unwrap());
    assert_eq!(("abc", 15), hex_char("Fabc").unwrap());
    assert_eq!(
        nom::Err::Error(("gabc", nom::error::ErrorKind::OneOf)),
        hex_char("gabc").unwrap_err()
    );
}

fn u16_char(i: &str) -> IResult<&str, char> {
    let (i, (_, _, c)) = tuple((char('\\'), char('u'), count(hex_char, 4)))(i)?;

    let c = char::try_from(c[0] * 0x1000 + c[1] * 0x0100 + c[2] * 0x0010 + c[3]).unwrap();
    Ok((i, c))
}

#[test]
fn test_u16_char() {
    assert_eq!(
        ("berlegenheit", 'Ü'),
        u16_char(r"\u00DCberlegenheit").unwrap()
    );
    assert!(u16_char(r"\uGGGG").is_err());
    assert!(u16_char(r"").is_err());
}

fn u32_char(i: &str) -> IResult<&str, char> {
    let (i, (_, _, c)) = tuple((char('\\'), char('U'), count(hex_char, 8)))(i)?;

    let c = c[0] * 0x10000000
        + c[1] * 0x01000000
        + c[2] * 0x00100000
        + c[3] * 0x00010000
        + c[4] * 0x00001000
        + c[5] * 0x00000100
        + c[6] * 0x00000010
        + c[7];
    if c <= 0x10FFFF {
        let c = char::try_from(c).unwrap();
        Ok((i, c))
    } else {
        Err(nom::Err::Failure((i, nom::error::ErrorKind::OneOf)))
    }
}

#[test]
fn test_u32_char() {
    assert_eq!(
        ("berlegenheit", 'Ü'),
        u32_char(r"\U000000DCberlegenheit").unwrap()
    );
    assert!(u16_char(r"\U0000GGGG").is_err());
    assert!(u16_char(r"\U0011FFFF").is_err());
    assert!(u16_char(r"").is_err());
}

fn iriref(i: &str) -> IResult<&str, String> {
    let iri_char = alt((none_of(r#"<>"{}|^`\"#), u16_char, u32_char));
    let (i, iri) = delimited(char('<'), many0(iri_char), char('>'))(i)?;
    let iri: String = iri.into_iter().collect();
    Ok((i, iri))
}

#[test]
fn test_iriref() {
    assert_eq!(("abc", String::from("https://google.com")), iriref("<https://google.com>abc").unwrap());
    assert_eq!(("abc", String::from("https://duckduckgo.com/?q=Übung")), iriref(r"<https://duckduckgo.com/?q=\u00DCbung>abc").unwrap());
}