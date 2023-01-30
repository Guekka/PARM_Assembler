use nom::bytes::complete::{tag_no_case, take_till, take_while};
use nom::character::complete::{char, line_ending, multispace1, space0};
use nom::combinator::{eof, map_opt, map_res, value};
use nom::error::{convert_error, ErrorKind, VerboseError};
use nom::multi::many_till;
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::{
    branch::alt,
    character::complete::digit1,
    combinator::{map, opt},
    sequence::tuple,
    Finish, IResult,
};
use regex::Regex;
use std::fmt::{Display, Formatter};
use thiserror::Error;

use crate::instructions::{Args, FullInstr, Immediate, Immediate8, Instr, Reg};
use crate::utils::{unescape_string, Appliable};

pub(crate) type Err<'a> = VerboseError<&'a str>;

trait Parseable: Sized {
    fn parse(input: &str) -> IResult<&str, Self, Err>;
}

impl Parseable for Reg {
    fn parse(input: &str) -> IResult<&str, Reg, Err> {
        let standard_reg = map_res(
            preceded(tag_no_case("r"), map_res(digit1, str::parse::<u8>)),
            Reg::try_from,
        );

        let sp = value(Reg::SP, tag_no_case("sp"));
        let pc = value(Reg::PC, tag_no_case("pc"));

        alt((standard_reg, sp, pc))(input)
    }
}

impl<const N: u8, const WIDE: bool> Parseable for Immediate<N, WIDE> {
    fn parse(input: &str) -> IResult<&str, Immediate<N, WIDE>, Err> {
        map_res(
            preceded(
                char('#'),
                map_res(take_while(|c: char| c.is_numeric()), str::parse::<u16>),
            ),
            Immediate::<N, WIDE>::new,
        )(input)
    }
}

fn parse_rd_rm_imm5(input: &str) -> IResult<&str, Args, Err> {
    map(
        tuple((
            preceded(parse_separator, Reg::parse),
            preceded(parse_separator, Reg::parse),
            preceded(parse_separator, Immediate::parse),
        )),
        Args::RdRmImm5.make_appliable(),
    )(input)
}

fn parse_rd_rn_rm(input: &str) -> IResult<&str, Args, Err> {
    map(
        tuple((
            preceded(parse_separator, Reg::parse),
            preceded(parse_separator, Reg::parse),
            preceded(parse_separator, Reg::parse),
        )),
        Args::RdRnRm.make_appliable(),
    )(input)
}

fn parse_rd_rn_imm3(input: &str) -> IResult<&str, Args, Err> {
    map(
        tuple((
            preceded(parse_separator, Reg::parse),
            preceded(parse_separator, Reg::parse),
            preceded(parse_separator, Immediate::parse),
        )),
        Args::RdRnImm3.make_appliable(),
    )(input)
}

fn parse_rd_imm8(input: &str) -> IResult<&str, Args, Err> {
    map(
        tuple((
            preceded(parse_separator, Reg::parse),
            preceded(parse_separator, Immediate::parse),
        )),
        Args::RdImm8.make_appliable(),
    )(input)
}

fn parse_sp_imm7(input: &str) -> IResult<&str, Args, Err> {
    map(
        preceded(
            tuple((parse_separator, tag_no_case("sp"), parse_separator)),
            Immediate::parse,
        ),
        Args::Immediate7W,
    )(input)
}

fn parse_two_regs(input: &str) -> IResult<&str, Args, Err> {
    map(
        tuple((
            preceded(parse_separator, Reg::parse),
            preceded(parse_separator, Reg::parse),
        )),
        Args::TwoRegs.make_appliable(),
    )(input)
}

fn parse_rdm_rn_rdm(input: &str) -> IResult<&str, Args, Err> {
    map_opt(
        tuple((
            preceded(parse_separator, Reg::parse),
            preceded(parse_separator, Reg::parse),
            preceded(parse_separator, Reg::parse),
        )),
        |(r1, r2, r1_)| {
            if r1 == r1_ {
                Some(Args::TwoRegs(r1, r2))
            } else {
                None
            }
        },
    )(input)
}

fn parse_rdrn_imm0(input: &str) -> IResult<&str, Args, Err> {
    map_opt(
        tuple((
            preceded(parse_separator, Reg::parse),
            preceded(parse_separator, Reg::parse),
            preceded(parse_separator, Immediate8::parse),
        )),
        |(rd, rn, imm0)| {
            if imm0.0 == 0 {
                Some(Args::RdRnImm0(rd, rn))
            } else {
                None
            }
        },
    )(input)
}

fn parse_rt_sp_imm8(input: &str) -> IResult<&str, Args, Err> {
    map(
        tuple((
            preceded(parse_separator, Reg::parse),
            preceded(
                parse_separator,
                delimited(
                    tag_no_case("[sp"),
                    map(opt(preceded(parse_separator, Immediate::parse)), |i| {
                        i.or_else(|| Some(Immediate::new(0).unwrap())).unwrap()
                    }),
                    char(']'),
                ),
            ),
        )),
        Args::RtSpImm8W.make_appliable(),
    )(input)
}

fn parse_rt_rn_imm5(input: &str) -> IResult<&str, Args, Err> {
    let inner_braces = pair(
        preceded(parse_separator, Reg::parse),
        opt(preceded(parse_separator, Immediate::parse)),
    );

    map(
        pair(
            preceded(parse_separator, Reg::parse),
            preceded(
                parse_separator,
                delimited(char('['), inner_braces, char(']')),
            ),
        ),
        |(rt, (rn, imm5))| {
            Args::RtRnImm5(rt, rn, imm5.unwrap_or_else(|| Immediate::new(0).unwrap()))
        },
    )(input)
}

fn parse_label(input: &str) -> IResult<&str, &str, Err> {
    preceded(
        opt(char('.')),
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    )(input)
}

fn parse_label_definition(input: &str) -> IResult<&str, &str, Err> {
    terminated(parse_label, char(':'))(input)
}

fn parse_label_args(input: &str) -> IResult<&str, Args, Err> {
    map(preceded(parse_separator, parse_label), |label| {
        Args::Label(label.to_owned())
    })(input)
}

fn parse_rt_label(input: &str) -> IResult<&str, Args, Err> {
    map(
        pair(
            preceded(parse_separator, Reg::parse),
            preceded(parse_separator, parse_label),
        ),
        |(reg, str)| Args::RtLabel(reg, str.to_owned()),
    )(input)
}

fn parse_separator(input: &str) -> IResult<&str, &str, Err> {
    preceded(opt(char(',')), space0)(input)
}

type ParseArgs = fn(&str) -> IResult<&str, Args, Err>;

/// The full list of supported instructions.
const INSTRUCTIONS: &[(Instr, ParseArgs); 50] = &[
    (Instr::Lsls, parse_rd_rm_imm5),
    (Instr::Lsrs, parse_rd_rm_imm5),
    (Instr::Asrs, parse_rd_rm_imm5),
    (Instr::Adds, parse_rd_rn_rm),
    (Instr::Adds2, parse_rd_rn_imm3),
    (Instr::Adds3, parse_rd_imm8),
    (Instr::Subs, parse_rd_rn_rm),
    (Instr::Subs2, parse_rd_rn_imm3),
    (Instr::Subs3, parse_rd_imm8),
    (Instr::Movs, parse_rd_imm8),
    (Instr::Rsbs, parse_rdrn_imm0),
    (Instr::Ands, parse_two_regs),
    (Instr::Eors, parse_two_regs),
    (Instr::Lsls2, parse_two_regs),
    (Instr::Lsrs2, parse_two_regs),
    (Instr::Asrs2, parse_two_regs),
    (Instr::Adcs, parse_two_regs),
    (Instr::Sbcs, parse_two_regs),
    (Instr::Rors, parse_two_regs),
    (Instr::Tst, parse_two_regs),
    (Instr::Rsbs, parse_two_regs),
    (Instr::Cmp, parse_two_regs),
    (Instr::Cmp2, parse_rd_imm8),
    (Instr::Cmn, parse_two_regs),
    (Instr::Orrs, parse_two_regs),
    (Instr::Muls, parse_rdm_rn_rdm),
    (Instr::Bics, parse_two_regs),
    (Instr::Mvns, parse_two_regs),
    (Instr::Str, parse_rt_sp_imm8),
    (Instr::Ldr, parse_rt_sp_imm8),
    (Instr::Ldr2, parse_rt_rn_imm5),
    (Instr::Ldr3, parse_rt_label),
    (Instr::AddSp, parse_sp_imm7),
    (Instr::SubSp, parse_sp_imm7),
    (Instr::Beq, parse_label_args),
    (Instr::Bne, parse_label_args),
    (Instr::Bcs, parse_label_args),
    (Instr::Bcc, parse_label_args),
    (Instr::Bmi, parse_label_args),
    (Instr::Bpl, parse_label_args),
    (Instr::Bvs, parse_label_args),
    (Instr::Bvc, parse_label_args),
    (Instr::Bhi, parse_label_args),
    (Instr::Bls, parse_label_args),
    (Instr::Bge, parse_label_args),
    (Instr::Blt, parse_label_args),
    (Instr::Bgt, parse_label_args),
    (Instr::Ble, parse_label_args),
    (Instr::Bal, parse_label_args),
    (Instr::B, parse_label_args),
];

/// Generates a parser for parsing the instructions
const fn generate_instructions_parser() -> fn(&str) -> IResult<&str, FullInstr, Err> {
    move |input: &str| {
        INSTRUCTIONS
            .iter()
            .flat_map(|(instr, parse_args)| {
                instr
                    .text_instruction()
                    .iter()
                    .map(|text_instr| (text_instr, instr, parse_args))
                    .collect::<Vec<_>>()
            })
            .map(|(&text_instr, instr, args_parser)| {
                map(
                    tuple((value(instr, tag_no_case(text_instr)), args_parser)),
                    |(instr, args)| FullInstr {
                        instr: *instr,
                        args,
                    },
                )
            })
            // we cannot nom::branch::alt here because it requires a tuple
            // so we manually implement the alt combinator
            .fold(
                Err(nom::Err::Error(nom::error::ParseError::from_error_kind(
                    input,
                    ErrorKind::Alt,
                ))),
                |acc, mut f| match acc {
                    Ok((i, o)) => Ok((i, o)),
                    Err(_) => f(input),
                },
            )
    }
}

/// Parses a single instruction.
fn parse_instr(input: &str) -> IResult<&str, FullInstr, Err> {
    const PARSE_INSTRUCTION: fn(&str) -> IResult<&str, FullInstr, Err> =
        generate_instructions_parser();
    PARSE_INSTRUCTION(input)
}

/// Handles `.asciz` (alias `.string`)
fn parse_string(input: &str) -> IResult<&str, String, Err> {
    let prefix = pair(
        alt((tag_no_case(".string"), tag_no_case(".asciz"))),
        pair(take_till(|c| c == '"'), char('"')),
    );

    let suffix = char('"');

    map(
        delimited(prefix, take_till(|c| c == '"'), suffix),
        unescape_string,
    )(input)
}

fn parse_comment(input: &str) -> IResult<&str, &str, Err> {
    preceded(preceded(space0, char('@')), take_till(|c| c == '\n'))(input)
}

fn parse_end_of_line(input: &str) -> IResult<&str, (), Err> {
    terminated(value((), space0), line_ending)(input)
}

/// clang emits push instructions that we don't support, so we just ignore them.
fn parse_push(input: &str) -> IResult<&str, (), Err> {
    value(
        (),
        delimited(tag_no_case("push"), take_till(|c| c == '\n'), line_ending),
    )(input)
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum ParsedLine {
    Instr(FullInstr),
    Label(String),
    String(String),
    None,
}

/// Parses a single line of assembly code.
/// A line can be an instruction, a label or a comment.
/// If the line is not an instruction or a label, it is ignored.
fn parse_line(input: &str) -> IResult<&str, ParsedLine, Err> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::ParseError::from_error_kind(
            input,
            ErrorKind::Eof,
        )));
    }
    terminated(
        alt((
            map(preceded(space0, parse_label_definition), |s| {
                ParsedLine::Label(s.to_owned())
            }),
            map(preceded(space0, parse_instr), ParsedLine::Instr),
            map(preceded(space0, parse_string), ParsedLine::String),
            value(ParsedLine::None, parse_push),
            value(ParsedLine::None, parse_comment),
            value(ParsedLine::None, multispace1),
            // If something starts with a dot and is not a label, it's probably a directive we can ignore
            value(
                ParsedLine::None,
                preceded(char('.'), take_till(|c| c == '\n')),
            ),
        )),
        opt(parse_end_of_line),
    )(input)
}

#[derive(Error, Debug)]
pub enum ParseError {
    NomError {
        errors: Vec<(String, ErrorKind)>,
        json: String,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::NomError { errors, json } => {
                writeln!(f, "Failed to parse assembly code:")?;
                for (line, error) in errors {
                    writeln!(f, "Error: {:?} at line: {}", error, line)?;
                }
                writeln!(f, "JSON: {}", json)
            }
        }
    }
}

impl ParseError {
    pub fn from_nom_error(input: &str, err: Err) -> Self {
        let json = convert_error(input, err.clone());

        let errors = err
            .errors
            .into_iter()
            .map(|(input, kind)| {
                let kind = match kind {
                    nom::error::VerboseErrorKind::Nom(nom_kind) => nom_kind,
                    _ => ErrorKind::Fail,
                };
                (input.lines().next().unwrap_or_default().to_owned(), kind)
            })
            .collect();

        Self::NomError { errors, json }
    }
}

fn preprocess(input: &str) -> String {
    const REPLACEMENTS: [(&str, &str); 2] = [
        (r#"movs?\s+(r\d), (r\d)"#, "lsls $1, $2, #0"),
        // let's hope nobody uses r6
        (
            r#"ldrb\s+(r\d), \[(r\d), (r\d)\]"#,
            "adds r6, $2, $3\nldrb $1, [r6]",
        ),
    ];

    let mut output = input.to_owned();

    for (pattern, replacement) in REPLACEMENTS.iter() {
        let re = Regex::new(pattern).unwrap();
        output = re.replace_all(&output, *replacement).to_string();
    }
    output
}
pub(crate) fn parse_lines(input: &str) -> Result<Vec<ParsedLine>, ParseError> {
    let input = preprocess(input);

    let res = many_till(parse_line, eof)(input.as_ref())
        .finish()
        .map(|(_, (lines, _))| lines)
        .map(|lines| {
            lines
                .into_iter()
                .filter(|l| l != &ParsedLine::None)
                .collect()
        })
        .map_err(|e| ParseError::from_nom_error(input.as_ref(), e));

    res
}

#[cfg(test)]
mod tests {
    use crate::instructions::Reg::R0;
    use crate::instructions::{Immediate3, Immediate5, Immediate7W, Immediate8, Immediate8W};

    use super::*;

    #[test]
    fn lsls() {
        let input = "lsls r0, r1, #4";
        let res = parse_instr(input).unwrap();
        let expected = FullInstr {
            instr: Instr::Lsls,
            args: Args::RdRmImm5(Reg::R0, Reg::R1, Immediate5::new(4).unwrap()),
        };
        assert_eq!(res.1, expected);
    }

    #[test]
    fn lsrs() {
        let input = "lsrs r2, r5, #9";
        let res = parse_instr(input).unwrap();
        let expected = FullInstr {
            instr: Instr::Lsrs,
            args: Args::RdRmImm5(Reg::R2, Reg::R5, Immediate5::new(9).unwrap()),
        };
        assert_eq!(res.1, expected);
    }

    #[test]
    fn asrs() {
        let input = "asrs r3, r7, #15";
        let res = parse_instr(input).unwrap();
        let expected = FullInstr {
            instr: Instr::Asrs,
            args: Args::RdRmImm5(Reg::R3, Reg::R7, Immediate5::new(15).unwrap()),
        };
        assert_eq!(res.1, expected);
    }

    #[test]
    fn invalid_register() {
        let input = "lsls r0, r8, #4";
        assert!(parse_instr(input).is_err());
    }

    #[test]
    fn parse_full_line() {
        let input = "lsls r0, r1, #4 @ comment";
        let res = parse_line(input).unwrap();
        let expected = ParsedLine::Instr(FullInstr {
            instr: Instr::Lsls,
            args: Args::RdRmImm5(Reg::R0, Reg::R1, Immediate5::new(4).unwrap()),
        });
        assert_eq!(res.1, expected);
    }

    #[test]
    fn parse_comments() {
        let input = "@ comment";
        let expected = ParsedLine::None;
        let res = parse_line(input).unwrap();
        assert_eq!(expected, res.1);
    }

    #[test]
    fn parse_whitespace_lines() {
        let input = "   ";
        let res = parse_line(input).unwrap();
        assert_eq!(ParsedLine::None, res.1);
    }

    #[test]
    fn parse_empty_lines() {
        let input = "";
        let res = parse_line(input);
        assert!(res.is_err());
    }

    #[test]
    fn parse_several_lines() {
        let input = "
            lsls r0, r1, #4
            @ comment
            lsrs r2, r5, #9 @comment
            
            asrs r3, r7, #15           
            ";
        let expected = &[
            ParsedLine::Instr(FullInstr {
                instr: Instr::Lsls,
                args: Args::RdRmImm5(Reg::R0, Reg::R1, Immediate5::new(4).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Lsrs,
                args: Args::RdRmImm5(Reg::R2, Reg::R5, Immediate5::new(9).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Asrs,
                args: Args::RdRmImm5(Reg::R3, Reg::R7, Immediate5::new(15).unwrap()),
            }),
        ];
        let res = parse_lines(input).unwrap();

        assert_eq!(expected, res.as_slice());
    }

    #[test]
    fn parse_label() {
        let input = ".label:";
        let expected = ParsedLine::Label("label".to_owned());
        let res = parse_line(input).unwrap();
        assert_eq!(expected, res.1);
    }

    #[test]
    fn parse_label_with_comment() {
        let input = ".label: @ comment";
        let expected = ParsedLine::Label("label".to_owned());
        let res = parse_line(input).unwrap();
        assert_eq!(expected, res.1);
    }

    #[test]
    fn parse_lines_with_label() {
        let input = "
            .label: lsls r0, r1, #4
            @ comment
            lsrs r2, r5, #9 @comment
            .label2:
            asrs r3, r7, #15           
            
            b .label2
            ";
        let expected = &[
            ParsedLine::Label("label".to_owned()),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Lsls,
                args: Args::RdRmImm5(Reg::R0, Reg::R1, Immediate5::new(4).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Lsrs,
                args: Args::RdRmImm5(Reg::R2, Reg::R5, Immediate5::new(9).unwrap()),
            }),
            ParsedLine::Label("label2".to_owned()),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Asrs,
                args: Args::RdRmImm5(Reg::R3, Reg::R7, Immediate5::new(15).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::B,
                args: Args::Label("label2".to_owned()),
            }),
        ];
        let res = parse_lines(input).unwrap();
        assert_eq!(expected, res.as_slice())
    }

    #[test]
    fn parse_branch() {
        let input = "b .label";
        let expected = ParsedLine::Instr(FullInstr {
            instr: Instr::B,
            args: Args::Label("label".to_owned()),
        });
        let res = parse_line(input).unwrap();
        assert_eq!(expected, res.1);
    }

    #[test]
    fn shift_add_sub_move() {
        let input = "
            movs r0, #0
            movs r1, #1
            movs r2, #170
            movs r3, #255
            
            
            lsls r4, r2, #1
            @r4 value should be 340, 154 
            
            lsrs r5, r2, #1
            @r5 value should be 85, 55 
            
            subs r6, r0, #5
            asrs r6, r6, #1
            @r6 value should be -3 ou FFFFFFFD
            
            adds r7, r6, r1 
            @r7 value should be -2, FFFFFFFE 
        ";
        use Reg::*;
        let expected: Vec<ParsedLine> = vec![
            ParsedLine::Instr(FullInstr {
                instr: Instr::Movs,
                args: Args::RdImm8(R0, Immediate8::new(0).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Movs,
                args: Args::RdImm8(R1, Immediate8::new(1).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Movs,
                args: Args::RdImm8(R2, Immediate8::new(170).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Movs,
                args: Args::RdImm8(R3, Immediate8::new(255).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Lsls,
                args: Args::RdRmImm5(R4, R2, Immediate5::new(1).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Lsrs,
                args: Args::RdRmImm5(R5, R2, Immediate5::new(1).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Subs2,
                args: Args::RdRnImm3(R6, R0, Immediate3::new(5).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Asrs,
                args: Args::RdRmImm5(R6, R6, Immediate5::new(1).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Adds,
                args: Args::RdRnRm(R7, R6, R1),
            }),
        ];
        let res = parse_lines(input).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn addsp() {
        let input = "
            add sp, #4
            sub sp, #100
        ";
        let expected: Vec<ParsedLine> = vec![
            ParsedLine::Instr(FullInstr {
                instr: Instr::AddSp,
                args: Args::Immediate7W(Immediate7W::new(4).unwrap()),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::SubSp,
                args: Args::Immediate7W(Immediate7W::new(100).unwrap()),
            }),
        ];
        let res = parse_lines(input).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn ldr() {
        let input = "ldr r2,[sp, #4]";
        let expected = ParsedLine::Instr(FullInstr {
            instr: Instr::Ldr,
            args: Args::RtSpImm8W(Reg::R2, Immediate8W::new(4).unwrap()),
        });
        let res = parse_line(input).unwrap();
        assert_eq!(expected, res.1);
    }

    #[test]
    fn sub() {
        let input = r#"
run:
	sub     sp, #96"#;

        let expected = vec![
            ParsedLine::Label("run".to_owned()),
            ParsedLine::Instr(FullInstr {
                instr: Instr::SubSp,
                args: Args::Immediate7W(Immediate7W::new(96).unwrap()),
            }),
        ];
        let res = parse_lines(input).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn str() {
        let input = "	str	r0, [sp]";

        let expected = ParsedLine::Instr(FullInstr {
            instr: Instr::Str,
            args: Args::RtSpImm8W(Reg::R0, Immediate8W::new(0).unwrap()),
        });

        let res = parse_line(input).unwrap();

        assert_eq!(expected, res.1);
    }

    #[test]
    fn string() {
        let input = r#".asciz  "  _____        _____  __  __\n |  __ \\ /\\   |  __ \\|  \\/  |\n | |__) /  \\  | |__) | \\  / |\n |  ___/ /\\ \\ |  _  /| |\\/| |\n | |  / ____ \\| | \\ \\| |  | |\n |_| /_/    \\_|_|  \\_|_|  |_|\n""#;

        let expected = ParsedLine::String("  _____        _____  __  __\n |  __ \\ /\\   |  __ \\|  \\/  |\n | |__) /  \\  | |__) | \\  / |\n |  ___/ /\\ \\ |  _  /| |\\/| |\n | |  / ____ \\| | \\ \\| |  | |\n |_| /_/    \\_|_|  \\_|_|  |_|\n".to_owned());

        let res = parse_line(input).unwrap();

        assert_eq!(expected, res.1);
    }

    #[test]
    fn ldr_label() {
        let input = "ldr     r0, .LCPI0_0";

        let expected = ParsedLine::Instr(FullInstr {
            instr: Instr::Ldr3,
            args: Args::RtLabel(R0, "LCPI0_0".to_owned()),
        });

        let actual = parse_line(input).unwrap();

        assert_eq!(actual.1, expected);
    }

    #[test]
    fn ldrb() {
        let input = "ldrb r0, [r1, #1]";

        let expected = ParsedLine::Instr(FullInstr {
            instr: Instr::Ldr2,
            args: Args::RtRnImm5(Reg::R0, Reg::R1, Immediate5::new(1).unwrap()),
        });

        let actual = parse_line(input).unwrap();

        assert_eq!(actual.1, expected);
    }
}
