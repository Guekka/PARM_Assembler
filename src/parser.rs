use crate::instructions::{Immediate8, Instr, RdRmImm5, Reg};
use crate::utils::Appliable;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{char, space0};
use nom::combinator::map_res;
use nom::sequence::preceded;
use nom::{
    branch::alt,
    character::complete::digit1,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

//const INSTRUCTIONS: [&str; 1] = [	"lsls {Rd}, {Rm}, {imm5}": (0b00000, "imm5", "Rm", "Rd"),];

fn parse_rm_rd_imm5(input: &str) -> IResult<&str, RdRmImm5> {
    map(
        tuple((
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_register),
            preceded(parse_separator, parse_immediate8bits),
        )),
        RdRmImm5.make_appliable(),
    )(input)
}

fn parse_register(input: &str) -> IResult<&str, Reg> {
    map_res(
        preceded(tag_no_case("r"), map_res(digit1, str::parse::<u8>)),
        Reg::try_from,
    )(input)
}

fn parse_immediate8bits(input: &str) -> IResult<&str, Immediate8> {
    map(
        preceded(char('#'), map_res(digit1, str::parse::<u8>)),
        Immediate8,
    )(input)
}

fn parse_separator(input: &str) -> IResult<&str, &str> {
    preceded(opt(char(',')), space0)(input)
}

fn parse_lsls(input: &str) -> IResult<&str, Instr> {
    map(preceded(tag_no_case("lsls"), parse_rm_rd_imm5), Instr::Lsls)(input)
}

pub fn parse_instr_line(input: &str) -> IResult<&str, Instr> {
    alt((parse_lsls,))(input)
}

#[test]
fn lsls() {
    let input = "lsls r0, r1, #4";
    let res = parse_instr_line(input).unwrap();
    let expected = Instr::Lsls(RdRmImm5(Reg::R0, Reg::R1, Immediate8(4)));
    assert_eq!(res.1, expected);
}
