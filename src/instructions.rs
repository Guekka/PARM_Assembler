use bitvec::bitvec;
use bitvec::prelude::Msb0;
use std::collections::HashMap;
use thiserror::Error;

#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub(crate) enum Reg {
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
pub(crate) enum ImmediateError {
    #[error("Immediate value {0} is too large")]
    TooLarge(i32),
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) struct Immediate<const N: u8, const WIDE: bool>(pub u16);

impl<const N: u8, const WIDE: bool> Immediate<N, WIDE> {
    const fn lower_bound() -> u16 {
        0
    }

    const fn upper_bound() -> u16 {
        let offset = if WIDE { 2 } else { 0 };
        (1 << (N + offset)) - 1
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
        let offset = if WIDE { 2 } else { 0 };
        -(1 << (N + offset - 1))
    }

    const fn upper_bound() -> i16 {
        let offset = if WIDE { 2 } else { 0 };
        (1 << (N + offset - 1)) - 1
    }

    pub(crate) fn new(val: i16) -> Result<Self, ImmediateError> {
        if val >= Self::lower_bound() && val <= Self::upper_bound() {
            Ok(Self(if WIDE { val / 4 } else { val }))
        } else {
            Err(ImmediateError::TooLarge(val as i32))
        }
    }
}

/// List of all possible instructions
#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum Instr {
    // Shift add sub move
    Lsls,
    Lsrs,
    Asrs,
    Adds,
    Subs,
    Adds2,
    Subs2,
    Movs,
    // Data processing
    Ands,
    Eors,
    Lsls2,
    Lsrs2,
    Asrs2,
    Adcs,
    Sbcs,
    Rors,
    Tst,
    Rsbs,
    Cmp,
    Cmn,
    Orrs,
    Muls,
    Bics,
    Mvns,
    // Load / Store
    Str,
    Ldr,
    // Misc
    AddSp,
    SubSp,
    Beq,
    Bne,
    Bcs,
    Bcc,
    Bmi,
    Bpl,
    Bvs,
    Bvc,
    Bhi,
    Bls,
    Bge,
    Blt,
    Bgt,
    Ble,
    Bal,
    B,
}

pub(crate) type BitVec = bitvec::prelude::BitVec<u8, Msb0>;

impl Instr {
    pub(crate) const fn text_instruction(&self) -> &'static str {
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
            Instr::AddSp => "add",
            Instr::SubSp => "sub",
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

    pub fn bits(&self) -> BitVec {
        use Instr::*;
        match &self {
            Lsls => bitvec![u8, Msb0; 0, 0, 0, 0, 0],
            Lsrs => bitvec![u8, Msb0; 0, 0, 0, 0, 1],
            Asrs => bitvec![u8, Msb0; 0, 0, 0, 1, 0],
            Adds => bitvec![u8, Msb0; 0, 0, 0, 1, 1, 0, 0],
            Subs => bitvec![u8, Msb0; 0, 0, 0, 1, 1, 0, 1],
            Adds2 => bitvec![u8, Msb0; 0, 0, 0, 1, 1, 1, 0],
            Subs2 => bitvec![u8, Msb0; 0, 0, 0, 1, 1, 1, 1],
            Movs => bitvec![u8, Msb0; 0, 0, 1, 0, 0],
            // Data processing
            Ands => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            Eors => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
            Lsls2 => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 0, 0, 1, 0],
            Lsrs2 => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 0, 0, 1, 1],
            Asrs2 => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
            Adcs => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 0, 1, 0, 1],
            Sbcs => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 0, 1, 1, 0],
            Rors => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 0, 1, 1, 1],
            Tst => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 0, 0, 0],
            Rsbs => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 0, 0, 1],
            Cmp => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 0, 1, 0],
            Cmn => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 0, 1, 1],
            Orrs => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 1, 0, 0],
            Muls => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 1, 0, 1],
            Bics => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 1, 1, 0],
            Mvns => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 1, 1, 1],
            // Load / Store
            Str => bitvec![u8, Msb0; 1, 0, 0, 1, 0],
            Ldr => bitvec![u8, Msb0; 1, 0, 0, 1, 1],
            // Misc
            AddSp => bitvec![u8, Msb0; 1, 0, 1, 1, 0, 0, 0, 0, 0],
            SubSp => bitvec![u8, Msb0; 1, 0, 1, 1, 0, 0, 0, 0, 1],
            Beq => bitvec![u8, Msb0; 1, 1, 0, 1, 0, 0, 0, 0],
            Bne => bitvec![u8, Msb0; 1, 1, 0, 1, 0, 0, 0, 1],
            Bcs => bitvec![u8, Msb0; 1, 1, 0, 1, 0, 0, 1, 0],
            Bcc => bitvec![u8, Msb0; 1, 1, 0, 1, 0, 0, 1, 1],
            Bmi => bitvec![u8, Msb0; 1, 1, 0, 1, 0, 1, 0, 0],
            Bpl => bitvec![u8, Msb0; 1, 1, 0, 1, 0, 1, 0, 1],
            Bvs => bitvec![u8, Msb0; 1, 1, 0, 1, 0, 1, 1, 0],
            Bvc => bitvec![u8, Msb0; 1, 1, 0, 1, 0, 1, 1, 1],
            Bhi => bitvec![u8, Msb0; 1, 1, 0, 1, 1, 0, 0, 0],
            Bls => bitvec![u8, Msb0; 1, 1, 0, 1, 1, 0, 0, 1],
            Bge => bitvec![u8, Msb0; 1, 1, 0, 1, 1, 0, 1, 0],
            Blt => bitvec![u8, Msb0; 1, 1, 0, 1, 1, 0, 1, 1],
            Bgt => bitvec![u8, Msb0; 1, 1, 0, 1, 1, 1, 0, 0],
            Ble => bitvec![u8, Msb0; 1, 1, 0, 1, 1, 1, 0, 1],
            Bal => bitvec![u8, Msb0; 1, 1, 0, 1, 1, 1, 1, 0],
            B => bitvec![u8, Msb0; 1, 1, 1, 0, 0],
        }
    }
}

pub(crate) type Immediate3 = Immediate<3, false>;
pub(crate) type Immediate5 = Immediate<5, false>;
pub(crate) type Immediate8 = Immediate<8, false>;
pub(crate) type Immediate11 = SignedImmediate<11, false>;

pub(crate) type Immediate8S = SignedImmediate<8, false>;

pub(crate) type Immediate7W = Immediate<7, true>;
pub(crate) type Immediate8W = Immediate<8, true>;

/// List of all possible instructions arguments
#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Args {
    Immediate11(Immediate11),
    Immediate7W(Immediate7W),
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
pub(crate) struct FullInstr {
    pub instr: Instr,
    pub args: Args,
}

pub(crate) type LabelLookup = HashMap<String, usize>;

#[derive(Error, Debug)]
pub enum CompleteError {
    #[error("Label {0} not found")]
    LabelNotFound(String),
    #[error("Label {label} is too far away: {distance}")]
    JumpTooFar { label: String, distance: i32 },
}

/// Complete the instruction by replacing labels with their actual address
/// conditional jumps can use 8 bits to encode the distance
fn complete_bcond(label: usize, cur_line: usize) -> Result<Args, CompleteError> {
    let offset = label as i16 - cur_line as i16 - 3;

    let imm = Immediate8S::new(offset).map_err(|_| CompleteError::JumpTooFar {
        label: label.to_string(),
        distance: offset as i32,
    })?;

    Ok(Args::Immediate8S(imm))
}


/// Complete the instruction by replacing labels with their actual address
/// Unconditional jumps can use 11 bits to encode the distance
fn complete_buncond(label: usize, cur_line: usize) -> Result<Args, CompleteError> {
    let offset = label as i16 - cur_line as i16 - 3;

    let imm = Immediate11::new(offset).map_err(|_| CompleteError::JumpTooFar {
        label: label.to_string(),
        distance: offset as i32,
    })?;

    Ok(Args::Immediate11(imm))
}

impl FullInstr {
    /// Complete the instruction by replacing labels with their actual address
    /// and checking that the jump is not too far away
    pub(crate) fn complete(
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
