mod emitter;
mod instructions;
mod logic;
mod parser;
mod utils;

use crate::instructions::CompleteError;
pub use crate::logic::make_program;
pub use crate::parser::parse_lines;

use thiserror::Error;

const HEADER: &str = "v2.0 raw\n";

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("Could not complete instruction: {0}")]
    CompleteError(#[from] CompleteError),
    #[error("Could not parse input: {0}")]
    ParseError(#[from] parser::ParseError),
}

pub fn export_to_logisim(input: &str) -> Result<String, ExportError> {
    let parsed = parse_lines(input)?;
    let program = make_program(parsed)?;

    let mut out = HEADER.to_owned();
    out.reserve(program.len() * 5);

    Ok(program
        .into_iter()
        .map(|i| format!("{:04x}", i))
        .fold(out, |acc, i| acc + &i + " ")
        .trim()
        .to_owned())
}
