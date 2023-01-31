use std::mem;
use thiserror::Error;

use crate::emitter::ToBinary;
use crate::instructions;
use crate::instructions::{BitVec, CompleteError, FullInstr, LabelLookup};
use crate::parser::ParsedLine;

/// Maps labels to their addresses.
/// The address of a label is the address of the instruction after the label.
fn calculate_labels(instrs: &[ParsedLine], ram: &[ParsedLine]) -> (LabelLookup, LabelLookup) {
    let rom_labels = instrs
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
        .collect();

    // RAM labels are a bit different: they need to account for string size
    let mut ram_labels = LabelLookup::new();
    let mut prev_string_end = 0;

    for line in ram.iter() {
        match line {
            ParsedLine::Label(label) => {
                ram_labels.insert(label.to_owned(), prev_string_end);
            }
            ParsedLine::String(string) => {
                prev_string_end += string.len();
            }
            _ => unreachable!("RAM should only contain labels and strings"),
        }
    }
    (rom_labels, ram_labels)
}

#[derive(Error, Debug)]
pub(crate) enum ProgramError {
    #[error("Could not complete instruction: {0}")]
    CompleteError(#[from] CompleteError),
}

fn extract_ram(instrs: &mut Vec<ParsedLine>) -> Vec<ParsedLine> {
    // strings are located after a label
    // so we need to find label immediately before a string
    let mut ram = Vec::new();
    let mut last_labels = Vec::new();
    let mut to_remove = Vec::new();

    for (i, instr) in instrs.iter().enumerate() {
        match instr {
            ParsedLine::Label(string) => {
                last_labels.push((i, string));
            }
            ParsedLine::String(string) => {
                if !last_labels.is_empty() {
                    for (i, label) in mem::take(&mut last_labels).into_iter() {
                        ram.push(ParsedLine::Label(label.to_owned()));
                        to_remove.push(i);
                    }
                    ram.push(ParsedLine::String(string.clone()));
                    to_remove.push(i);
                } else {
                    panic!("String without label: {}", string);
                }
            }
            _ => last_labels.clear(),
        }
    }

    for i in to_remove.iter().rev() {
        instrs.remove(*i);
    }

    ram
}

/// Replaces ldr rt, label with ldr rt, another label
/// Used for cases like:
/// ```asm
/// label:
///    .long another_label
/// ```
// TODO: this is a bit hacky, maybe there is a better way to do this
fn collapse_long(instrs: &mut Vec<ParsedLine>) {
    let mut to_remove = Vec::new();

    for i in 1..instrs.len() {
        // if we have a long...
        if let ParsedLine::Long(long_label) = instrs[i].clone() {
            // ...and a label before it...
            if let ParsedLine::Label(label) = instrs[i - 1].clone() {
                // ...replace all ldr rt, label with ldr rt, long_label
                for instr in instrs.iter_mut() {
                    if let ParsedLine::Instr(FullInstr {
                        instr: instructions::Instr::Ldr3,
                        args: instructions::Args::RtLabel(_, ldr_label),
                    }) = instr
                    {
                        if ldr_label == &label {
                            *ldr_label = long_label.clone();
                        }
                    }
                }
                to_remove.push(i);
            }
        }
    }

    for i in to_remove.iter().rev() {
        instrs.remove(*i);
    }
}

fn process_lines(
    mut instrs: Vec<ParsedLine>,
    ram: &[ParsedLine],
) -> Result<(Vec<FullInstr>, Vec<String>), CompleteError> {
    collapse_long(&mut instrs);

    let (rom_labels, ram_labels) = calculate_labels(&instrs, ram);

    let only_instrs = instrs
        .iter_mut()
        .filter_map(|l| match l {
            ParsedLine::Instr(i) => Some(i),
            _ => None,
        })
        .enumerate()
        .map(|(i, instr)| instr.complete(i, &rom_labels, &ram_labels))
        .collect::<Result<_, _>>()?;

    let ram = ram
        .iter()
        .filter_map(|l| match l {
            ParsedLine::String(s) => Some(s),
            _ => None,
        })
        .map(|s| s.to_owned())
        .collect();

    Ok((only_instrs, ram))
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub(crate) struct Program {
    pub(crate) instrs: BitVec,
    pub(crate) ram: BitVec,
}

pub(crate) fn make_program(mut instrs: Vec<ParsedLine>) -> Result<Program, CompleteError> {
    let ram = extract_ram(&mut instrs);

    let (rom, ram) = process_lines(instrs, &ram)?;

    let rom = rom.into_iter().fold(BitVec::new(), |mut acc, instr| {
        acc.extend(instr.to_binary());
        acc
    });

    let ram = ram.into_iter().fold(BitVec::new(), |mut acc, string| {
        acc.extend(string.to_binary());
        acc
    });

    Ok(Program { instrs: rom, ram })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unusual_byte_groupings)]

    use crate::instructions::Reg::{R0, R1, R4, R5};
    use crate::instructions::{Args, FullInstr, Immediate5, Instr};
    use bitvec::bitvec;
    use bitvec::order::Msb0;

    use super::*;

    #[test]
    fn test_simple_program() {
        let instrs = vec![ParsedLine::Instr(FullInstr {
            instr: Instr::Lsrs,
            args: Args::RdRmImm5(R0, R1, Immediate5::new(5).unwrap()),
        })];

        let expected = bitvec![u8, Msb0;
            0, 0, 0, 0, 1,
            0, 0, 1, 0, 1,
            0, 0, 1,
            0, 0, 0];

        let program = make_program(instrs).unwrap();
        assert_eq!(program.instrs, expected);
        assert!(program.ram.is_empty());
    }

    #[test]
    fn test_make_program_with_labels() {
        let mut instrs = vec![
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

        let ram = extract_ram(&mut instrs);

        let (rom_labels, ram_labels) = calculate_labels(&instrs, &ram);
        let expected_labels: LabelLookup = vec![("label1".to_owned(), 0), ("label2".to_owned(), 3)]
            .into_iter()
            .collect();

        assert_eq!(rom_labels, expected_labels);
        assert!(ram_labels.is_empty());

        let program = make_program(instrs).unwrap();

        let expected_rom = bitvec![u8, Msb0;
            0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, //
            1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, // bal to label at line 3
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, //
            1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, // bal to label at line 0
        ];

        assert_eq!(program.instrs, expected_rom);
        assert!(program.ram.is_empty());
    }

    #[test]
    fn unexisting_label() {
        let instrs = vec![ParsedLine::Instr(FullInstr {
            instr: Instr::B,
            args: Args::Label("label".to_owned()),
        })];
        let program = make_program(instrs);
        assert!(program.is_err());
    }

    #[test]
    fn use_ram() {
        let instrs = vec![
            ParsedLine::Instr(FullInstr {
                instr: Instr::Ldr3,
                args: Args::RtLabel(R0, "label".to_owned()),
            }),
            ParsedLine::Label("label".to_owned()),
            ParsedLine::String("Hello".to_owned()),
            ParsedLine::Label("label2".to_owned()),
        ];

        let program = make_program(instrs).unwrap();

        let expected_rom = bitvec![u8, Msb0;
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // movs r0, #0
        ];

        let expected_ram = bitvec![u8, Msb0;
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1,
        ];

        assert_eq!(
            program,
            Program {
                instrs: expected_rom,
                ram: expected_ram
            }
        );
    }
}
