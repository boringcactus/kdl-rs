use std::ops::RangeTo;
use std::{collections::HashMap, iter::from_fn};

use crate::nom_compat::{many0, many1, many_till};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_until1, take_while_m_n};
use nom::character::complete::{anychar, char, none_of, one_of};
use nom::combinator::{
    all_consuming, eof, iterator, map, map_opt, map_res, not, opt, recognize, value,
};
use nom::error::ParseError;
use nom::multi::fold_many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::{IResult, Offset, Parser, Slice};

use crate::{
    KdlComment, KdlDocument, KdlEntity, KdlNode, KdlParseError, KdlString, KdlType, KdlValue,
};

pub(crate) fn document(input: &str) -> IResult<&str, KdlDocument, KdlParseError<&str>> {
    map(many0(entity), KdlDocument)(input)
}

fn entity(input: &str) -> IResult<&str, KdlEntity, KdlParseError<&str>> {
    alt((
        map(whitespace, KdlEntity::Whitespace),
        map(comment, KdlEntity::Comment),
        map(node, |(ty, node)| KdlEntity::Node(KdlType(ty), node)),
    ))(input)
}

fn whitespace(input: &str) -> IResult<&str, String, KdlParseError<&str>> {
    map(
        recognize(many1(alt((unicode_space, newline)))),
        String::from,
    )(input)
}

fn unicode_space(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
    alt((
        tag(" "),
        tag("\t"),
        tag("\u{FEFF}"), // BOM
        tag("\u{00A0}"),
        tag("\u{1680}"),
        tag("\u{2000}"),
        tag("\u{2001}"),
        tag("\u{2002}"),
        tag("\u{2003}"),
        tag("\u{2004}"),
        tag("\u{2005}"),
        tag("\u{2006}"),
        tag("\u{2007}"),
        tag("\u{2008}"),
        tag("\u{2009}"),
        tag("\u{200A}"),
        tag("\u{202F}"),
        tag("\u{205F}"),
        tag("\u{3000}"),
    ))(input)
}

/// `newline := All line-break unicode white_space
pub(crate) fn newline(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
    alt((
        tag("\r\n"),
        tag("\r"),
        tag("\n"),
        tag("\u{0085}"),
        tag("\u{000C}"),
        tag("\u{2028}"),
        tag("\u{2029}"),
    ))(input)
}

fn comment(input: &str) -> IResult<&str, KdlComment, KdlParseError<&str>> {
    alt((
        map(single_line_comment, |x| {
            KdlComment::SingleLine(String::from(x))
        }),
        map(multi_line_comment, |x| {
            KdlComment::MultiLine(String::from(x))
        }),
        map(slash_dash_comment, |x| {
            KdlComment::SlashDash(String::from(x))
        }),
    ))(input)
}

/// `single-line-comment := '//' ('\r' [^\n] | [^\r\n])* (newline | eof)`
fn single_line_comment(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
    recognize(preceded(tag("//"), many_till(anychar, alt((newline, eof)))))(input)
}

/// `multi-line-comment := '/*' commented-block
fn multi_line_comment(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
    recognize(preceded(tag("/*"), commented_block))(input)
}

/// `commented-block := '*/' | (multi-line-comment | '*' | '/' | [^*/]+) commented-block`
fn commented_block(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
    alt((
        tag("*/"),
        terminated(
            alt((multi_line_comment, take_until1("*/"), tag("*"), tag("/"))),
            commented_block,
        ),
    ))(input)
}

fn slash_dash_comment(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
    recognize(preceded(
        tag("/-"),
        // TODO: alt((node, etc, etc))
        node,
    ))(input)
}

fn node(input: &str) -> IResult<&str, (String, KdlNode), KdlParseError<&str>> {
    todo!()
}

fn boolean(input: &str) -> IResult<&str, KdlValue, KdlParseError<&str>> {
    alt((
        map(tag("true"), |_| KdlValue::Bool(true)),
        map(tag("false"), |_| KdlValue::Bool(false)),
    ))(input)
}

fn null(input: &str) -> IResult<&str, KdlValue, KdlParseError<&str>> {
    map(tag("null"), |_| KdlValue::Null)(input)
}

/// `escaped-string := '"' character* '"'`
fn string(input: &str) -> IResult<&str, KdlValue, KdlParseError<&str>> {
    let (input, _) = tag("\"")(input)?;
    let mut original = String::new();
    let mut value = String::new();
    original.push('"');
    let (input, chars) = many0(character)(input)?;
    for (raw, processed) in chars {
        original.push_str(raw);
        value.push(processed);
    }
    let (input, _) = tag("\"")(input)?;
    original.push('"');
    Ok((input, KdlValue::String(KdlString { original, value })))
}

/// `character := '\' escape | [^\"]`
fn character(input: &str) -> IResult<&str, (&str, char), KdlParseError<&str>> {
    with_raw(alt((preceded(char('\\'), escape), none_of("\\\""))))(input)
}

/// This is like `recognize`, but _also_ returns the actual value.
fn with_raw<I: Clone + Offset + Slice<RangeTo<usize>>, O, E: ParseError<I>, F>(
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, (I, O), E>
where
    F: Parser<I, O, E>,
{
    move |input: I| {
        let i = input.clone();
        match parser.parse(i) {
            Ok((i, x)) => {
                let index = input.offset(&i);
                Ok((i, (input.slice(..index), x)))
            }
            Err(e) => Err(e),
        }
    }
}

// creates a (map, inverse map) tuple
macro_rules! bimap {
    ($($x:expr => $y:expr),+) => {
        (phf::phf_map!($($x => $y),+), phf::phf_map!($($y => $x),+))
    }
}

/// a map and its inverse of escape-sequence<->char
pub(crate) static ESCAPE_CHARS: (phf::Map<char, char>, phf::Map<char, char>) = bimap! {
    '"' => '"',
    '\\' => '\\',
    '/' => '/',
    'b' => '\u{08}',
    'f' => '\u{0C}',
    'n' => '\n',
    'r' => '\r',
    't' => '\t'
};

/// `escape := ["\\/bfnrt] | 'u{' hex-digit{1, 6} '}'`
fn escape(input: &str) -> IResult<&str, char, KdlParseError<&str>> {
    alt((
        delimited(tag("u{"), unicode, char('}')),
        map_opt(anychar, |c| ESCAPE_CHARS.0.get(&c).copied()),
    ))(input)
}

fn unicode(input: &str) -> IResult<&str, char, KdlParseError<&str>> {
    map_opt(
        map_res(
            take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
            |hex| u32::from_str_radix(hex, 16),
        ),
        std::char::from_u32,
    )(input)
}

#[cfg(test)]
mod whitespace_tests {
    #[test]
    fn basic() {
        use super::whitespace;

        assert_eq!(whitespace(" \t\n\r"), Ok(("", String::from(" \t\n\r"))));
    }
}

#[cfg(test)]
mod comment_tests {
    use super::*;

    #[test]
    fn single_line() {
        assert_eq!(
            comment("// Hello world"),
            Ok(("", KdlComment::SingleLine("// Hello world".into())))
        );
    }

    #[test]
    fn multi_line() {
        assert_eq!(
            comment("/* Hello world */"),
            Ok(("", KdlComment::MultiLine("/* Hello world */".into())))
        );
    }
}

#[cfg(test)]
mod value_tests {
    use super::*;

    #[test]
    fn boolean_val() {
        assert_eq!(boolean("true"), Ok(("", KdlValue::Bool(true))));
        assert_eq!(boolean("false"), Ok(("", KdlValue::Bool(false))));
    }

    #[test]
    fn null_val() {
        assert_eq!(null("null"), Ok(("", KdlValue::Null)));
    }

    #[test]
    fn string_val() {
        assert_eq!(
            string(r#""Hello \n\u{2020}world""#),
            Ok((
                "",
                KdlValue::String(KdlString {
                    original: r#""Hello \n\u{2020}world""#.into(),
                    value: "Hello \n\u{2020}world".into()
                })
            ))
        );
    }
}
