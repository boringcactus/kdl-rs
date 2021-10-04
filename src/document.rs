use std::iter::from_fn;

use nom::character::complete::anychar;
use nom::combinator::{all_consuming, eof, iterator, not, value};
use nom::sequence::terminated;
use nom::Finish;

use crate::nom_compat::many_till;

use crate::{KdlEntity, KdlError, KdlErrorKind};

use crate::parser::{document, newline};

pub struct KdlDocument(pub(crate) Vec<KdlEntity>);

impl KdlDocument {
    /// Parse a KDL document from a string into a [`KdlDocument`] object model.
    pub fn parse(input: &str) -> Result<KdlDocument, KdlError> {
        all_consuming(document)(input)
            .finish()
            .map(|(_, arg)| arg)
            .map_err(|e| {
                let prefix = &input[..(input.len() - e.input.len())];
                let (line, column) = calculate_line_column(prefix);
                KdlError {
                    input: input.into(),
                    offset: prefix.chars().count(),
                    line,
                    column,
                    kind: if let Some(kind) = e.kind {
                        kind
                    } else if let Some(ctx) = e.context {
                        KdlErrorKind::Context(ctx)
                    } else {
                        KdlErrorKind::Other
                    },
                }
            })
    }
}

fn calculate_line_column(input: &str) -> (usize, usize) {
    let (input, skipped_lines) = count_leading_lines(input);
    let input = strip_trailing_newline(input);
    (skipped_lines + 1, input.len() + 1) // +1 as we're 1-based
}

// The following two functions exist for the purposes of translating offsets into line/column pairs
// for error reporting. We're doing this here so we can make use of our `newline` definition, to
// ensure line/column information is reported accurately based on our definition of newlines, even
// if we update our definition of newlines later.

/// Counts all lines in the input up to the final line.
///
/// This counts and skips past all lines terminated in `newline` with the exception of the final
/// line, regardless of whether it's newline-terminated. If the input only contains a single line,
/// the input will be returned unmodified with a count of `0`.
pub(crate) fn count_leading_lines(input: &str) -> (&str, usize) {
    let mut iter = iterator(
        input,
        terminated(many_till(value((), anychar), newline), not(eof)),
    );
    let count = (&mut iter).count();
    match iter.finish().finish() {
        Ok((input, _)) => (input, count),
        // I don't believe this particular parser can error, but we need to handle it anyway
        Err(e) => (e.input, count),
    }
}

/// Strips a single trailing `newline`, if present, from the input.
pub(crate) fn strip_trailing_newline(input: &str) -> &str {
    // Nom doesn't support parsing in reverse, but we want to reuse our newline definition. The
    // longest newline sequence is 2 characters, so we can just test the last char, and the
    // second-to-last char, and validate that the parser consumes the full input.
    let mut idx_iter = input.char_indices().map(|(idx, _)| idx);
    let mut last = idx_iter.next_back();
    let mut second_last = idx_iter.next_back();
    // Start with the second-to-last, otherwise \r\n will be parsed as just the \n.
    from_fn(|| second_last.take().or_else(|| last.take()))
        .find(|&idx| all_consuming(newline)(&input[idx..]).is_ok())
        .map(|idx| &input[..idx])
        .unwrap_or(input)
}
