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

use crate::{KdlDocument, KdlEntity, KdlParseError};

pub(crate) fn document(input: &str) -> IResult<&str, KdlDocument, KdlParseError<&str>> {
    map(many0(entity), KdlDocument)(input)
}

fn entity(input: &str) -> IResult<&str, KdlEntity, KdlParseError<&str>> {
    alt((whitespace, comment, node))(input)
}

fn whitespace(input: &str) -> IResult<&str, KdlEntity, KdlParseError<&str>> {
    todo!()
}

fn comment(input: &str) -> IResult<&str, KdlEntity, KdlParseError<&str>> {
    todo!()
}

fn node(input: &str) -> IResult<&str, KdlEntity, KdlParseError<&str>> {
    todo!()
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
