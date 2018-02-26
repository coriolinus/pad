//! `pad` handles padding and alignment within delimited strings
//!
//! While its primary use case is as a command line tool, this library
//! is provided in case a programmatic interface might be useful to someone.

use std::cmp::max;
use std::io;

extern crate itertools;
use itertools::{Itertools, Position};

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;


/// A Cell is a single field within a row
///
/// It is a string slice and the grapheme count within that string slice
pub type Cell<'input> = (&'input str, usize);

/// A Row is a list of Cells.
pub type Row<'input> = Vec<Cell<'input>>;

/// Split an input row into its component graphemes
///
/// This is more complex than it may appear at first glance, because we
/// properly handle Unicode grapheme clusters.
///
/// This function makes no attempt to split rows; that should be handled
/// externally.
pub fn split_row<'input>(input: &'input str, delimiter: char) -> Row<'input> {
    let mut row = Row::new();

    for substr in input.split(delimiter) {
        let graphemes = substr.graphemes(true).count();
        row.push((substr, graphemes));
    }

    row
}

pub type Document<'input> = Vec<Row<'input>>;

/// Parse an input string into a document
///
/// Note that this must retain references to the entire input string, which
/// effectively means that the entire input must remain in memory before output
/// can begin. This may cause problems for large inputs.
pub fn parse_document<'input>(input: &'input str, delimiter: char) -> Document<'input> {
    input.lines().map(|line| split_row(line, delimiter)).collect()
}

/// Alignment controls the alignment of the output within a column
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

impl Default for Alignment {
    fn default() -> Alignment {
        Alignment::Left
    }
}

/// A Column is a basic unit of output
pub struct Column {
    alignment: Alignment,
    width: usize,
}

/// This struct configures the output of a document
pub struct OutputConfig {
    separator: char,
    columns: Vec<Column>,
}

/// Generate output configuration from the given document
pub fn configure_output(
    document: &Document,
    alignments: &[Alignment],
    output_separator: char,
) -> OutputConfig {
    let mut output = OutputConfig{
        separator: output_separator,
        columns: Vec::new(),
    };

    for row in document {
        for (idx, &(_, width)) in row.iter().enumerate() {
            if idx < output.columns.len() {
                // this column exists
                output.columns[idx].width = max(output.columns[idx].width, width);
            } else {
                output.columns.push(Column{
                    alignment: if idx < alignments.len() { alignments[idx] } else { Alignment::default() },
                    width: width,
                })
            }
        }
    }

    output
}

/// Write a column to the given writer
fn write_col<W: io::Write>(writer: &mut W, text: &str, width: usize, alignment: Alignment) -> io::Result<()> {
    match alignment {
        Alignment::Left => write!(writer, "{: <width$}", text, width=width)?,
        Alignment::Center => write!(writer, "{: ^width$}", text, width=width)?,
        Alignment::Right => write!(writer, "{: >width$}", text, width=width)?,
    }
    Ok(())
}

/// Write the given document to the specified writer using the given output configuration
pub fn write<'input, W: io::Write>(writer: &mut W, document: &Document<'input>, config: &OutputConfig, autoflush: bool) -> io::Result<()> {
    for row in document {
        for position in row.iter().zip(config.columns.iter()).with_position() {
            match position {
                Position::First((&(text, _), column)) | Position::Middle((&(text, _), column)) => {
                    write_col(writer, text, column.width, column.alignment)?;
                    write!(writer, "{}", config.separator)?;
                }

                Position::Last((&(text, _), column)) | Position::Only((&(text, _), column)) => {
                    write_col(writer, text, column.width, column.alignment)?;
                    writeln!(writer, "")?;
                }
            }
        }
        if autoflush {
            writer.flush()?;
        }
    }
    Ok(())
}

/// Print the given document to stdout using the given output configuration
pub fn print<'input>(document: &Document<'input>, config: &OutputConfig) -> io::Result<()> {
    let mut stdout = io::stdout();
    write(&mut stdout, document, config, true)
}