use crate::instructions::{
    Args, FullInstr, Immediate3, Immediate5, Immediate7W, Immediate8, Immediate8W, Instr,
    ParsedLine, RdImm8, RdRmImm5, RdRnImm0, RdRnImm3, RdRnRm, Reg, RnRm, RtSpImm8W,
};
use nom::bytes::complete::{tag_no_case, take_till, take_while};
use nom::character::complete::{char, line_ending, space0};
use nom::combinator::{map_opt, map_res, value};
use nom::error::{ErrorKind, ParseError, VerboseError};
use nom::multi::many1;
use nom::sequence::{delimited, preceded, terminated};
use nom::{
    branch::alt,
    character::complete::digit1,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

type Err<'a> = VerboseError<&'a str>;

fn parse_rd_rm_imm5(input: &str) -> IResult<&str, Args, Err> {
    map(
        tuple((
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_immediate5bits),
        )),
        |(rd, rm, imm5)| Args::RdRmImm5(RdRmImm5(rd, rm, imm5)),
    )(input)
}

fn parse_rd_rn_rm(input: &str) -> IResult<&str, Args, Err> {
    map(
        tuple((
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_register),
        )),
        |(rd, rn, rm)| Args::RdRnRm(RdRnRm(rd, rn, rm)),
    )(input)
}

fn parse_rd_rn_imm3(input: &str) -> IResult<&str, Args, Err> {
    map(
        tuple((
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_immediate3bits),
        )),
        |(rd, rn, imm3)| Args::RdRnImm3(RdRnImm3(rd, rn, imm3)),
    )(input)
}

fn parse_rd_imm8(input: &str) -> IResult<&str, Args, Err> {
    map(
        tuple((
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_immediate8bits),
        )),
        |(rd, imm8)| Args::RdImm8(RdImm8(rd, imm8)),
    )(input)
}

fn parse_imm7(input: &str) -> IResult<&str, Args, Err> {
    map(preceded(parse_separator, parse_immediate7bits), |imm7| {
        Args::Immediate7W(imm7)
    })(input)
}

fn parse_rmrn(input: &str) -> IResult<&str, Args, Err> {
    map(
        tuple((
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_register),
        )),
        |(rm, rn)| Args::RnRm(RnRm(rm, rn)),
    )(input)
}

fn parse_rdrn_imm0(input: &str) -> IResult<&str, Args, Err> {
    map_opt(
        tuple((
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_immediate8bits),
        )),
        |(rd, rn, imm0)| {
            if imm0.0 == 0 {
                Some(Args::RdRnImm0(RdRnImm0(rd, rn)))
            } else {
                None
            }
        },
    )(input)
}

fn parse_rt_sp_imm8(input: &str) -> IResult<&str, Args, Err> {
    map(
        tuple((
            preceded(parse_separator, parse_register),
            preceded(
                parse_separator,
                delimited(
                    tag_no_case("[sp"),
                    preceded(parse_separator, parse_immediate8bits_w),
                    char(']'),
                ),
            ),
        )),
        |(rt, imm8)| Args::RtSpImm8W(RtSpImm8W(rt, imm8)),
    )(input)
}

fn parse_register(input: &str) -> IResult<&str, Reg, Err> {
    map_res(
        preceded(tag_no_case("r"), map_res(digit1, str::parse::<u8>)),
        Reg::try_from,
    )(input)
}

fn parse_immediate5bits(input: &str) -> IResult<&str, Immediate5, Err> {
    map(
        preceded(char('#'), map_res(digit1, str::parse::<u8>)),
        Immediate5,
    )(input)
}

fn parse_immediate7bits(input: &str) -> IResult<&str, Immediate7W, Err> {
    map(
        preceded(char('#'), map_res(digit1, str::parse::<u8>)),
        Immediate7W,
    )(input)
}

fn parse_immediate3bits(input: &str) -> IResult<&str, Immediate3, Err> {
    map(
        preceded(char('#'), map_res(digit1, str::parse::<u8>)),
        Immediate3,
    )(input)
}

fn parse_immediate8bits(input: &str) -> IResult<&str, Immediate8, Err> {
    map(
        preceded(char('#'), map_res(digit1, str::parse::<u8>)),
        Immediate8,
    )(input)
}
fn parse_immediate8bits_w(input: &str) -> IResult<&str, Immediate8W, Err> {
    map(
        preceded(char('#'), map_res(digit1, str::parse::<u8>)),
        Immediate8W,
    )(input)
}

fn parse_label(input: &str) -> IResult<&str, &str, Err> {
    preceded(char('.'), take_while(|c: char| c.is_alphanumeric()))(input)
}

fn parse_label_definition(input: &str) -> IResult<&str, &str, Err> {
    terminated(parse_label, char(':'))(input)
}

fn parse_label_args(input: &str) -> IResult<&str, Args, Err> {
    map(preceded(parse_separator, parse_label), |label| {
        Args::Label(label.to_owned())
    })(input)
}

fn parse_separator(input: &str) -> IResult<&str, &str, Err> {
    preceded(opt(char(',')), space0)(input)
}

const INSTRUCTIONS: &[(Instr, fn(&str) -> IResult<&str, Args, Err>); 30] = &[
    (Instr::Lsls, parse_rd_rm_imm5),
    (Instr::Lsrs, parse_rd_rm_imm5),
    (Instr::Asrs, parse_rd_rm_imm5),
    (Instr::Adds, parse_rd_rn_rm),
    (Instr::Subs, parse_rd_rn_rm),
    (Instr::Adds2, parse_rd_rn_imm3),
    (Instr::Subs2, parse_rd_rn_imm3),
    (Instr::Movs, parse_rd_imm8),
    (Instr::Cmp, parse_rmrn),
    (Instr::Rsbs, parse_rdrn_imm0),
    (Instr::Str, parse_rt_sp_imm8),
    (Instr::Ldr, parse_rt_sp_imm8),
    (Instr::AddSp, parse_imm7),
    (Instr::SubSp, parse_imm7),
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

const fn generate_instructions_parser() -> fn(&str) -> IResult<&str, FullInstr, Err> {
    move |input: &str| {
        INSTRUCTIONS
            .iter()
            .map(|(instr, args_parser)| {
                map(
                    tuple((
                        value(instr, tag_no_case(instr.text_instruction())),
                        args_parser,
                    )),
                    |(instr, args)| FullInstr {
                        instr: *instr,
                        args: args,
                    },
                )
            })
            // we cannot nom::branch::alt here because it requires a tuple
            // so we manually implement the alt combinator
            .fold(
                Err(nom::Err::Error(Err::from_error_kind(input, ErrorKind::Alt))),
                |acc, mut f| match acc {
                    Ok((i, o)) => Ok((i, o)),
                    Err(_) => f(input),
                },
            )
    }
}

const PARSE_INSTRUCTION: fn(&str) -> IResult<&str, FullInstr, Err> = generate_instructions_parser();

pub fn parse_instr(input: &str) -> IResult<&str, FullInstr, Err> {
    PARSE_INSTRUCTION(input)
}

pub fn parse_comment(input: &str) -> IResult<&str, &str, Err> {
    preceded(preceded(space0, char('@')), take_till(|c| c == '\n'))(input)
}

pub fn parse_end_of_line(input: &str) -> IResult<&str, (), Err> {
    terminated(value((), space0), line_ending)(input)
}

pub fn parse_line(input: &str) -> IResult<&str, ParsedLine, Err> {
    if input.is_empty() {
        return Err(nom::Err::Error(VerboseError { errors: vec![] }));
    }
    terminated(
        alt((
            map(preceded(space0, parse_label_definition), |s| {
                ParsedLine::Label(s.to_owned())
            }),
            map(preceded(space0, parse_instr), ParsedLine::Instr),
            value(ParsedLine::None, parse_comment),
            value(ParsedLine::None, space0),
        )),
        opt(parse_end_of_line),
    )(input)
}

pub fn parse_lines(input: &str) -> IResult<&str, Vec<ParsedLine>, Err> {
    many1(parse_line)(input).map(|(i, o)| {
        (
            i,
            o.into_iter().filter(|l| l != &ParsedLine::None).collect(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::convert_error;
    use nom::Finish;

    #[test]
    fn lsls() {
        let input = "lsls r0, r1, #4";
        let res = parse_instr(input).unwrap();
        let expected = FullInstr {
            instr: Instr::Lsls,
            args: Args::RdRmImm5(RdRmImm5(Reg::R0, Reg::R1, Immediate5(4))),
        };
        assert_eq!(res.1, expected);
    }

    #[test]
    fn lsrs() {
        let input = "lsrs r2, r5, #9";
        let res = parse_instr(input).unwrap();
        let expected = FullInstr {
            instr: Instr::Lsrs,
            args: Args::RdRmImm5(RdRmImm5(Reg::R2, Reg::R5, Immediate5(9))),
        };
        assert_eq!(res.1, expected);
    }

    #[test]
    fn asrs() {
        let input = "asrs r3, r7, #15";
        let res = parse_instr(input).unwrap();
        let expected = FullInstr {
            instr: Instr::Asrs,
            args: Args::RdRmImm5(RdRmImm5(Reg::R3, Reg::R7, Immediate5(15))),
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
            args: Args::RdRmImm5(RdRmImm5(Reg::R0, Reg::R1, Immediate5(4))),
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
        let expected =
            IResult::<&str, ParsedLine, Err>::Err(nom::Err::Error(VerboseError { errors: vec![] }));
        let res = parse_line(input);
        assert_eq!(expected, res);
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
                args: Args::RdRmImm5(RdRmImm5(Reg::R0, Reg::R1, Immediate5(4))),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Lsrs,
                args: Args::RdRmImm5(RdRmImm5(Reg::R2, Reg::R5, Immediate5(9))),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Asrs,
                args: Args::RdRmImm5(RdRmImm5(Reg::R3, Reg::R7, Immediate5(15))),
            }),
        ];
        let res = parse_lines(input);

        match res.finish() {
            Ok((_, lines)) => assert_eq!(expected, lines.as_slice()),
            Err(e) => panic!("Error: {}", convert_error(input, e)),
        }
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
                args: Args::RdRmImm5(RdRmImm5(Reg::R0, Reg::R1, Immediate5(4))),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Lsrs,
                args: Args::RdRmImm5(RdRmImm5(Reg::R2, Reg::R5, Immediate5(9))),
            }),
            ParsedLine::Label("label2".to_owned()),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Asrs,
                args: Args::RdRmImm5(RdRmImm5(Reg::R3, Reg::R7, Immediate5(15))),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::B,
                args: Args::Label("label2".to_owned()),
            }),
        ];
        let res = parse_lines(input).unwrap();
        assert_eq!(expected, res.1.as_slice())
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
                args: Args::RdImm8(RdImm8(R0, Immediate8(0))),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Movs,
                args: Args::RdImm8(RdImm8(R1, Immediate8(1))),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Movs,
                args: Args::RdImm8(RdImm8(R2, Immediate8(170))),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Movs,
                args: Args::RdImm8(RdImm8(R3, Immediate8(255))),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Lsls,
                args: Args::RdRmImm5(RdRmImm5(R4, R2, Immediate5(1))),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Lsrs,
                args: Args::RdRmImm5(RdRmImm5(R5, R2, Immediate5(1))),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Subs2,
                args: Args::RdRnImm3(RdRnImm3(R6, R0, Immediate3(5))),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Asrs,
                args: Args::RdRmImm5(RdRmImm5(R6, R6, Immediate5(1))),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::Adds,
                args: Args::RdRnRm(RdRnRm(R7, R6, R1)),
            }),
        ];
        let res = parse_lines(input).unwrap();
        assert_eq!(expected, res.1);
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
                args: Args::Immediate7W(Immediate7W(4)),
            }),
            ParsedLine::Instr(FullInstr {
                instr: Instr::SubSp,
                args: Args::Immediate7W(Immediate7W(100)),
            }),
        ];
        let res = parse_lines(input).unwrap();
        assert_eq!(expected, res.1);
    }

    #[test]
    fn ldr() {
        let input = "ldr r2,[sp, #4]";
        let expected = ParsedLine::Instr(FullInstr {
            instr: Instr::Ldr,
            args: Args::RtSpImm8W(RtSpImm8W(Reg::R2, Immediate8W(4))),
        });
        let res = parse_line(input).unwrap();
        assert_eq!(expected, res.1);
    }
}
