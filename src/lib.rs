mod emitter;
mod instructions;
mod logic;
mod parser;
mod utils;

pub use crate::logic::make_program;
pub use crate::parser::parse_lines;

const HEADER: &str = "v2.0 raw\n";

pub fn export_to_logisim(input: &str) -> Result<String, String> {
    let parsed = parse_lines(input).unwrap();
    let program = make_program(parsed.1)?;

    let mut out = HEADER.to_owned();
    out.reserve(program.len() * 5);

    Ok(program
        .into_iter()
        .map(|i| format!("{:04x}", i))
        .fold(out, |acc, i| acc + &i + " ")
        .trim()
        .to_owned())
}
