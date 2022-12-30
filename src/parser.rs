use crate::instructions::{Args, FullInstr, Immediate5, Instr, ParsedLine, RdRmImm5, Reg};
use crate::utils::Appliable;
use nom::bytes::complete::{tag_no_case, take_till};
use nom::character::complete::{char, line_ending, space0};
use nom::combinator::{map_res, value};
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::{preceded, terminated};
use nom::{
    branch::alt,
    character::complete::digit1,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

type Err<'a> = VerboseError<&'a str>;

fn parse_rm_rd_imm5(input: &str) -> IResult<&str, RdRmImm5, Err> {
    map(
        tuple((
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_immediate5bits),
        )),
        RdRmImm5.make_appliable(),
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

fn parse_separator(input: &str) -> IResult<&str, &str, Err> {
    preceded(opt(char(',')), space0)(input)
}

fn parse_lsls(input: &str) -> IResult<&str, FullInstr, Err> {
    map(preceded(tag_no_case("lsls"), parse_rm_rd_imm5), |args| {
        FullInstr {
            instr: Instr::Lsls,
            args: Args::RdRmImm5(args),
        }
    })(input)
}

fn parse_lsrs(input: &str) -> IResult<&str, FullInstr, Err> {
    map(preceded(tag_no_case("lsrs"), parse_rm_rd_imm5), |args| {
        FullInstr {
            instr: Instr::Lsrs,
            args: Args::RdRmImm5(args),
        }
    })(input)
}

fn parse_asrs(input: &str) -> IResult<&str, FullInstr, Err> {
    map(preceded(tag_no_case("asrs"), parse_rm_rd_imm5), |args| {
        FullInstr {
            instr: Instr::Asrs,
            args: Args::RdRmImm5(args),
        }
    })(input)
}

pub fn parse_instr(input: &str) -> IResult<&str, FullInstr, Err> {
    alt((parse_lsls, parse_lsrs, parse_asrs))(input)
}

pub fn parse_comment(input: &str) -> IResult<&str, &str, Err> {
    preceded(preceded(space0, char('@')), take_till(|c| c == '\n'))(input)
}

pub fn parse_end_of_line(input: &str) -> IResult<&str, (), Err> {
    terminated(value((), take_till(|c| c == '\n')), line_ending)(input)
}

pub fn parse_line(input: &str) -> IResult<&str, ParsedLine, Err> {
    if input.is_empty() {
        return Err(nom::Err::Error(VerboseError { errors: vec![] }));
    }
    terminated(
        alt((
            map(preceded(space0, parse_instr), ParsedLine::Instr),
            value(ParsedLine::None, parse_comment),
            value(ParsedLine::None, space0),
        )),
        terminated(opt(parse_comment), opt(parse_end_of_line)),
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
}
