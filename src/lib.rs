//! PARM Assembler is a library for assembling a subset of the ARM instruction set.
//! It is designed to be used in conjunction with logisim-evolution, a digital logic simulator.
//! It was written as a part of the Computer Architecture course.
//!
//! # Examples
//! ```
//!         let input = "
//!             movs r0, #0
//!             movs r1, #1
//!             .goto:
//!             movs r2, #20
//!             cmp r0, r1
//!             bMI .then1
//!             b .endif1
//!             .then1:
//!             rsbs r2, r2, #0
//!             .endif1:
//!             cmp r2, r1
//!             bLT .then2
//!             b .endif2
//!             .then2:
//!             movs r0, #50
//!             b .goto
//!             .endif2:
//!             adds r3, r0, r2
//!             @a comment";
//!
//!         let output = parm_assembler::export_to_logisim(input).unwrap();
//!
//!         // logisim-evolution expects the output to be in the following format:
//!         let expected = "v2.0 raw\n2000 2101 2214 4288 d4ff e7ff 4252 428a dbff e000 2032 e7f4 1883";
//!
//!         assert_eq!(expected, output.rom);
//! ```
//!
//! # Supported instructions
//!
//! See the [assignment](https://bitbucket.org/edge-team-leat/parm_public/src/21ae509e77e4e70bc79301eb59c3f1f9567fb62e/doc/main.pdf) for a full list of supported instructions.
//!
//! # Overview
//!
//! - Each instruction is an enum variant.
//! - Each instruction is associated to a struct representing its operands.
//! - Each operand is an enum variant.
//! - Lines are parsed into a vector of `Instruction`s using the `nom` crate.
//! - The `Instruction`s are then converted into a byte vector using the `bitvec` crate, each one being 16 bits long.
//! - The byte vector is then converted into a string of hexadecimal numbers.

use bitvec::field::BitField;
use thiserror::Error;

use crate::instructions::{BitVec, CompleteError};
pub use crate::logic::make_program;
pub use crate::parser::parse_lines;

mod emitter;
mod instructions;
mod logic;
mod parser;
mod utils;

pub const LOGISIM_HEADER: &str = "v2.0 raw\n";

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("Could not complete instruction: {0}")]
    CompleteError(#[from] CompleteError),
    #[error("Could not parse input: {0}")]
    ParseError(#[from] parser::ParseError),
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct LogisimProgram {
    pub rom: String,
    pub ram: String,
}

impl LogisimProgram {
    pub fn with_rom(rom: String) -> Self {
        Self {
            rom,
            ram: LOGISIM_HEADER.trim().to_owned(),
        }
    }
}

fn convert_to_logisim(data: BitVec) -> String {
    let mut out = LOGISIM_HEADER.to_owned();
    out.reserve(data.len() * 5);

    data.chunks(16)
        .into_iter()
        .map(|chunk| chunk.load_be::<u16>())
        .map(|integer| format!("{integer:04x}"))
        .fold(out, |acc, i| acc + &i + " ")
        .trim()
        .to_owned()
}

/// Assembles the given lines of assembly code into a binary program in logisim format.
///
/// # Arguments
///
/// * `input`: A list of ARM instructions, one per line.
///
/// returns: A string containing the binary representation of the program, in logisim format.
pub fn export_to_logisim(input: &str) -> Result<LogisimProgram, ExportError> {
    let parsed = parse_lines(input)?;
    let program = make_program(parsed)?;

    Ok(LogisimProgram {
        rom: convert_to_logisim(program.instrs),
        ram: convert_to_logisim(program.ram),
    })
}
