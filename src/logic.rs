use bitvec::field::BitField;
use thiserror::Error;

use crate::emitter::ToBinary;
use crate::instructions::{Args, BitVec, CompleteError, FullInstr, LabelLookup};
use crate::parser::ParsedLine;

/// Maps labels to their addresses.
/// The address of a label is the address of the instruction after the label.
fn calculate_labels(instrs: &[ParsedLine]) -> LabelLookup {
    instrs
        .iter()
        .enumerate()
        .filter_map(|(i, l)| match l {
            ParsedLine::Label(l) => Some((i, l.to_owned())),
            ParsedLine::Instr(FullInstr {
                args: Args::RtLabel(_, label),
                ..
            }) => Some((i, label.clone())),
            _ => None,
        })
        .enumerate()
        // this is a bit tricky: labels do not have an address on their own
        // so we need to substract current label index
        .map(|(label_i, (i, l))| (l, i - label_i))
        .collect()
}

#[derive(Error, Debug)]
pub(crate) enum ProgramError {
    #[error("Could not complete instruction: {0}")]
    CompleteError(#[from] CompleteError),
}

enum ProcessedLine {
    Instr(FullInstr),
    String(String),
}

impl ToBinary for ProcessedLine {
    fn to_binary(&self) -> BitVec {
        match self {
            ProcessedLine::Instr(instr) => instr.to_binary(),
            ProcessedLine::String(string) => string.to_binary(),
        }
    }
}

fn process_lines(mut instrs: Vec<ParsedLine>) -> Result<Vec<ProcessedLine>, CompleteError> {
    let labels = calculate_labels(&instrs);

    let only_instrs = instrs
        .iter_mut()
        .filter_map(|l| match l {
            ParsedLine::Instr(i) => Some(i),
            _ => None,
        })
        .enumerate();

    for (i, instr) in only_instrs {
        *instr = instr.complete(i, &labels)?;
    }

    Ok(instrs
        .into_iter()
        .filter_map(|l| match l {
            ParsedLine::Instr(i) => Some(ProcessedLine::Instr(i)),
            ParsedLine::String(s) => Some(ProcessedLine::String(s)),
            _ => None,
        })
        .collect())
}

pub(crate) fn make_program(instrs: Vec<ParsedLine>) -> Result<Vec<u16>, CompleteError> {
    let res = process_lines(instrs)?
        .into_iter()
        .map(|line| line.to_binary())
        .flat_map(|line| {
            let chunks = line.chunks_exact(16);
            // logisim uses big endian
            chunks.map(|c| c.load_be::<u16>()).collect::<Vec<_>>()
        })
        .collect();

    Ok(res)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unusual_byte_groupings)]

    use crate::instructions::Args::Label;
    use crate::instructions::Reg::{R0, R1, R4, R5};
    use crate::instructions::{Args, FullInstr, Immediate5, Instr};

    use super::*;

    #[test]
    fn test_simple_program() {
        let instrs = vec![ParsedLine::Instr(FullInstr {
            instr: Instr::Lsrs,
            args: Args::RdRmImm5(R0, R1, Immediate5::new(5).unwrap()),
        })];

        let program = make_program(instrs).unwrap();
        assert_eq!(program, vec![0b00001_00101_001_000]);
    }

    #[test]
    fn test_make_program_with_labels() {
        let instrs = vec![
            ParsedLine::Label("label1".to_owned()),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Lsrs,
                args: Args::RdRmImm5(R0, R1, Immediate5::new(5).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Bal,
                args: Args::Label("label2".to_owned()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Lsls,
                args: Args::RdRmImm5(R4, R5, Immediate5::new(2).unwrap()),
            }),
            ParsedLine::Label("label2".to_owned()),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Bal,
                args: Args::Label("label1".to_owned()),
            }),
        ];

        let labels = calculate_labels(&instrs);
        let expected_labels: LabelLookup = vec![("label1".to_owned(), 0), ("label2".to_owned(), 3)]
            .into_iter()
            .collect();

        assert_eq!(labels, expected_labels);

        let program = make_program(instrs).unwrap();
        assert_eq!(
            program,
            vec![
                0b00001_00101_001_000,
                0b11011110_11111111, // bal to label at line 3
                0b00000_00010_101_100,
                0b11011110_11111010, // bal to label at line 0
            ]
        );
    }

    #[test]
    fn unexisting_label() {
        let instrs = vec![ParsedLine::Instr(FullInstr {
            instr: Instr::B,
            args: Label("label".to_owned()),
        })];
        let program = make_program(instrs);
        assert!(program.is_err());
    }
}
