use bitvec::prelude::*;
use thiserror::Error;

use crate::emitter::ToBinary;
use crate::instructions::{CompleteError, FullInstr, LabelLookup, LiteralPool, LiteralPoolBuilder, LiteralPoolError};
use crate::parser::ParsedLine;


/// Maps labels to their addresses.
/// The address of a label is the address of the instruction after the label.
fn calculate_labels(instrs: &[ParsedLine]) -> LabelLookup {
    instrs
        .iter()
        .enumerate()
        .filter_map(|(i, l)| match l {
            ParsedLine::Label(l) => Some((i, l.to_owned())),
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
    Pool(LiteralPool),
}

impl ToBinary for ProcessedLine {
    fn to_binary(&self) -> crate::instructions::BitVec {
        match self {
            ProcessedLine::Instr(instr) => instr.to_binary(),
            ProcessedLine::Pool(pool) => pool.to_binary()
        }
    }
}

fn calculate_literal_pools(instrs: &Vec<ParsedLine>) -> Result<Vec<LiteralPool>, LiteralPoolError> {
    instrs.iter().filter_map(|instr| match instr {
        ParsedLine::String(s) => Some(s),
        _ => None,
    }).fold(LiteralPoolBuilder::new(), |mut acc, str| {
        acc.add(str.clone());
        acc
    }).make_pools()
}

fn process_lines(instrs: Vec<ParsedLine>) -> Result<Vec<ProcessedLine>, CompleteError> {
    let labels = calculate_labels(&instrs);
    let literal_pools = calculate_literal_pools(&instrs).expect("test");

    let instrs =
        instrs
            .into_iter()
            .filter_map(|l| match l {
                ParsedLine::Instr(i) => Some(i),
                _ => None,
            }).collect::<Vec<_>>();

    let mut completed_instrs = instrs
        .into_iter().enumerate()
        .map(|(i, instr)| instr.complete(i, &labels)).map(|res| res.map(|instr| ProcessedLine::Instr(instr))).collect::<Result<Vec<_>, _>>()?;


    if !literal_pools.is_empty() {
        // let's find an unreachable place
        // I think we're supposed to split pools between unreachable places... but we'll just use the first place
        // and hope everything's fine
        let first_unreachable = completed_instrs.iter().enumerate().filter(|(_, line)| {
            if let ProcessedLine::Instr(instr) = line {
                instr.instr.next_is_unreachable()
            } else {
                false
            }
        }).next().expect("There should be at least one unreachable place in the code").0;

        for pool in literal_pools {
            completed_instrs.insert(first_unreachable, ProcessedLine::Pool(pool));
        }
    }

    Ok(completed_instrs)
}

pub(crate) fn make_program(instrs: Vec<ParsedLine>) -> Result<Vec<u16>, CompleteError> {
    let res = process_lines(instrs)?.into_iter().map(|line| line.to_binary())
        // logisim uses big endian
        .map(|bits| bits.load_be::<u16>())
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
