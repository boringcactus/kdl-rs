use std::{collections::HashMap, iter::from_fn};

use crate::nom_compat::{many0, many1, many_till};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_until1, take_while_m_n};
use nom::character::complete::{anychar, char, none_of, one_of};
use nom::combinator::{
    all_consuming, eof, iterator, map, map_opt, map_res, not, opt, recognize, value,
};
use nom::multi::fold_many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

use crate::{KdlComment, KdlDocument, KdlEntity, KdlNode, KdlParseError, KdlType};

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
        map(slash_dash_node_comment, |x| {
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

fn slash_dash_node_comment(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
    recognize(preceded(tag("/-"), node))(input)
}

fn node(input: &str) -> IResult<&str, (String, KdlNode), KdlParseError<&str>> {
    todo!()
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
}
