use std::collections::HashMap;
use thiserror::Error;

#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Reg {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    PC = 14,
    SP = 15,
}

impl TryFrom<u8> for Reg {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        for &reg in [
            Reg::R0,
            Reg::R1,
            Reg::R2,
            Reg::R3,
            Reg::R4,
            Reg::R5,
            Reg::R6,
            Reg::R7,
            Reg::PC,
            Reg::SP,
        ]
        .iter()
        {
            if reg as u8 == value {
                return Ok(reg);
            }
        }
        Err(())
    }
}

#[derive(Error, Debug)]
pub enum ImmediateError {
    #[error("Immediate value {0} is too large")]
    TooLarge(i32),
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Immediate<const N: u8, const WIDE: bool>(pub u16);

impl<const N: u8, const WIDE: bool> Immediate<N, WIDE> {
    const fn lower_bound() -> u16 {
        0
    }

    const fn upper_bound() -> u16 {
        (1 << N) - 1
    }

    pub(crate) fn new(val: u16) -> Result<Self, ImmediateError> {
        if val >= Self::lower_bound() && val <= Self::upper_bound() {
            Ok(Self(if WIDE { val / 4 } else { val }))
        } else {
            Err(ImmediateError::TooLarge(val as i32))
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SignedImmediate<const N: u8, const WIDE: bool>(pub i16);

impl<const N: u8, const WIDE: bool> SignedImmediate<N, WIDE> {
    const fn lower_bound() -> i16 {
        -(1 << (N - 1))
    }

    const fn upper_bound() -> i16 {
        (1 << (N - 1)) - 1
    }

    pub(crate) fn new(val: i16) -> Result<Self, ImmediateError> {
        if val >= Self::lower_bound() && val <= Self::upper_bound() {
            Ok(Self(if WIDE { val / 4 } else { val }))
        } else {
            Err(ImmediateError::TooLarge(val as i32))
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u16)]
pub enum Instr {
    // Shift add sub move
    Lsls = 0,
    Lsrs = 1,
    Asrs = 2,
    Adds = 12,
    Subs = 13,
    Adds2 = 14,
    Subs2 = 15,
    Movs = 4,
    // Data processing
    Ands = 0b010000_0000,
    Eors = 0b010000_0001,
    Lsls2 = 0b010000_0010,
    Lsrs2 = 0b010000_0011,
    Asrs2 = 0b010000_0100,
    Adcs = 0b010000_0101,
    Sbcs = 0b010000_0110,
    Rors = 0b010000_0111,
    Tst = 0b010000_1000,
    Rsbs = 0b010000_1001,
    Cmp = 0b010000_1010,
    Cmn = 0b010000_1011,
    Orrs = 0b010000_1100,
    Muls = 0b010000_1101,
    Bics = 0b010000_1110,
    Mvns = 0b010000_1111,
    // Load / Store
    Str = 0b1001_0,
    Ldr = 0b1001_1,
    // Misc
    AddSp = 0b1011_00000,
    SubSp = 0b1011_00001,
    Beq = 0b1101_0000,
    Bne = 0b1101_0001,
    Bcs = 0b1101_0010,
    Bcc = 0b1101_0011,
    Bmi = 0b1101_0100,
    Bpl = 0b1101_0101,
    Bvs = 0b1101_0110,
    Bvc = 0b1101_0111,
    Bhi = 0b1101_1000,
    Bls = 0b1101_1001,
    Bge = 0b1101_1010,
    Blt = 0b1101_1011,
    Bgt = 0b1101_1100,
    Ble = 0b1101_1101,
    Bal = 0b1101_1110,
    B = 0b11100,
}

impl Instr {
    pub const fn text_instruction(&self) -> &'static str {
        match self {
            Instr::Lsls => "lsls",
            Instr::Lsrs => "lsrs",
            Instr::Asrs => "asrs",
            Instr::Adds => "adds",
            Instr::Subs => "subs",
            Instr::Adds2 => "adds",
            Instr::Subs2 => "subs",
            Instr::Movs => "movs",
            Instr::Str => "str",
            Instr::Ldr => "ldr",
            Instr::AddSp => "add sp",
            Instr::SubSp => "sub sp",
            Instr::Ands => "ands",
            Instr::Eors => "eors",
            Instr::Lsls2 => "lsls",
            Instr::Lsrs2 => "lsrs",
            Instr::Asrs2 => "asrs",
            Instr::Adcs => "adcs",
            Instr::Sbcs => "sbcs",
            Instr::Rors => "rors",
            Instr::Tst => "tst",
            Instr::Rsbs => "rsbs",
            Instr::Cmp => "cmp",
            Instr::Cmn => "cmn",
            Instr::Orrs => "orrs",
            Instr::Muls => "muls",
            Instr::Bics => "bics",
            Instr::Mvns => "mvns",
            Instr::B => "b",
            Instr::Beq => "beq",
            Instr::Bne => "bne",
            Instr::Bcs => "bcs",
            Instr::Bcc => "bcc",
            Instr::Bmi => "bmi",
            Instr::Bpl => "bpl",
            Instr::Bvs => "bvs",
            Instr::Bvc => "bvc",
            Instr::Bhi => "bhi",
            Instr::Bls => "bls",
            Instr::Bge => "bge",
            Instr::Blt => "blt",
            Instr::Bgt => "bgt",
            Instr::Ble => "ble",
            Instr::Bal => "bal",
        }
    }
}

pub type Immediate3 = Immediate<3, false>;
pub type Immediate5 = Immediate<5, false>;
pub type Immediate8 = Immediate<8, false>;
pub type Immediate11 = SignedImmediate<11, false>;

pub type Immediate8S = SignedImmediate<8, false>;

pub type Immediate7W = Immediate<7, true>;
pub type Immediate8W = Immediate<8, true>;

#[derive(PartialEq, Debug, Clone)]
pub enum Args {
    Immediate11(Immediate11),
    Immediate7W(Immediate7W),
    Immediate8(Immediate8),
    Immediate8S(Immediate8S),
    Label(String),
    RdImm8(Reg, Immediate8),
    RdRmImm5(Reg, Reg, Immediate5),
    RdRnImm0(Reg, Reg),
    RdRnImm3(Reg, Reg, Immediate3),
    RdRnRm(Reg, Reg, Reg),
    RtSpImm8W(Reg, Immediate8W),
    TwoRegs(Reg, Reg),
}

#[derive(PartialEq, Debug, Clone)]
pub struct FullInstr {
    pub instr: Instr,
    pub args: Args,
}

pub type LabelLookup = HashMap<String, usize>;

#[derive(Error, Debug)]
pub enum CompleteError {
    #[error("Label {0} not found")]
    LabelNotFound(String),
    #[error("Label {label} is too far away: {distance}")]
    JumpTooFar { label: String, distance: i32 },
}

fn complete_bcond(label: usize, cur_line: usize) -> Result<Args, CompleteError> {
    let offset = label as i16 - cur_line as i16 - 3;

    let imm = Immediate8S::new(offset).map_err(|_| CompleteError::JumpTooFar {
        label: label.to_string(),
        distance: offset as i32,
    })?;

    Ok(Args::Immediate8S(imm))
}

fn complete_buncond(label: usize, cur_line: usize) -> Result<Args, CompleteError> {
    let offset = label as i16 - cur_line as i16 - 3;

    let imm = Immediate11::new(offset).map_err(|_| CompleteError::JumpTooFar {
        label: label.to_string(),
        distance: offset as i32,
    })?;

    Ok(Args::Immediate11(imm))
}

impl FullInstr {
    pub fn complete(
        &self,
        cur_line: usize,
        labels: &LabelLookup,
    ) -> Result<FullInstr, CompleteError> {
        let mut copy = self.clone();
        if let Args::Label(ref label) = self.args {
            if let Some(&addr) = labels.get(label) {
                copy.args = match self {
                    FullInstr {
                        instr: Instr::B, ..
                    } => complete_buncond(addr, cur_line)?,
                    _ => complete_bcond(addr, cur_line)?,
                }
            } else {
                return Err(CompleteError::LabelNotFound(label.clone()));
            }
        }
        Ok(copy)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ParsedLine {
    Instr(FullInstr),
    Label(String),
    None,
}
