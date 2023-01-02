use crate::emitter::ToBinary;
use crate::instructions::{CompleteError, LabelLookup};
use crate::parser::ParsedLine;
use bitvec::prelude::*;
use thiserror::Error;

pub fn calculate_labels(instrs: &[ParsedLine]) -> LabelLookup {
    instrs
        .iter()
        .enumerate()
        .filter_map(|(i, l)| match l {
            ParsedLine::Label(l) => Some((i, l.to_owned())),
            _ => None,
        })
        .enumerate()
        .map(|(label_i, (i, l))| (l, i - label_i))
        .collect()
}

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("Could not complete instruction: {0}")]
    CompleteError(#[from] CompleteError),
}

pub fn make_program(instrs: Vec<ParsedLine>) -> Result<Vec<u16>, CompleteError> {
    let labels = calculate_labels(&instrs);

    instrs
        .into_iter()
        .filter_map(|l| match l {
            ParsedLine::Instr(i) => Some(i),
            _ => None,
        })
        .inspect(|i| println!("0: {:?}", i))
        .enumerate()
        .map(|(i, instr)| instr.complete(i, &labels))
        .inspect(|i| println!("1: {:?}", i))
        .map(|r| r.map(|i| i.to_binary()))
        .inspect(|r| println!("2: {:?}", r))
        .map(|r| r.map(|b| b.load_be::<u16>()))
        .inspect(|r| println!("3: {:?}", r))
        .collect::<Result<Vec<u16>, CompleteError>>()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unusual_byte_groupings)]
    use super::*;
    use crate::instructions::Args::Label;
    use crate::instructions::Reg::{R0, R1, R4, R5};
    use crate::instructions::{Args, FullInstr, Immediate5, Instr};

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
                0b11011110_11111010 // bal to label at line 0
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
