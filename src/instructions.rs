use std::collections::HashMap;

use bitvec::bitvec;
use bitvec::prelude::Msb0;
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
        let offset = if WIDE { 2 } else { 0 };
        1 << (N + offset)
    }

    pub fn new(val: u16) -> Result<Self, ImmediateError> {
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
pub enum Instr {
    // Shift add sub move
    Lsls,
    Lsrs,
    Asrs,
    Adds,
    Subs,
    Adds2,
    Subs2,
    Adds3,
    Subs3,
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
    Cmp2,
    Cmn,
    Orrs,
    Muls,
    Bics,
    Mvns,
    // Load / Store
    Str,
    Ldr,
    Ldr2,
    Ldr3,
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

pub type BitVec = bitvec::prelude::BitVec<u8, Msb0>;

impl Instr {
    pub(crate) fn text_instruction(&self) -> &'static [&'static str] {
        match self {
            Instr::Lsls => &["lsls"],
            Instr::Lsrs => &["lsrs"],
            Instr::Asrs => &["asrs"],
            Instr::Adds => &["adds"],
            Instr::Subs => &["subs"],
            Instr::Adds2 => &["adds", "add"],
            Instr::Subs2 => &["subs", "sub"],
            Instr::Adds3 => &["adds"],
            Instr::Subs3 => &["subs"],
            Instr::Movs => &["movs"],
            Instr::Str => &["str"],
            Instr::Ldr => &["ldr"],
            Instr::Ldr2 => &["ldr", "ldrb"],
            Instr::Ldr3 => &["ldr"],
            Instr::AddSp => &["add"],
            Instr::SubSp => &["sub"],
            Instr::Ands => &["ands"],
            Instr::Eors => &["eors"],
            Instr::Lsls2 => &["lsls"],
            Instr::Lsrs2 => &["lsrs"],
            Instr::Asrs2 => &["asrs"],
            Instr::Adcs => &["adcs"],
            Instr::Sbcs => &["sbcs"],
            Instr::Rors => &["rors"],
            Instr::Tst => &["tst"],
            Instr::Rsbs => &["rsbs"],
            Instr::Cmp => &["cmp"],
            Instr::Cmp2 => &["cmp"],
            Instr::Cmn => &["cmn"],
            Instr::Orrs => &["orrs"],
            Instr::Muls => &["muls"],
            Instr::Bics => &["bics"],
            Instr::Mvns => &["mvns"],
            Instr::B => &["b"],
            Instr::Beq => &["beq"],
            Instr::Bne => &["bne"],
            Instr::Bcs => &["bcs", "bhs"],
            Instr::Bcc => &["bcc", "blo"],
            Instr::Bmi => &["bmi"],
            Instr::Bpl => &["bpl"],
            Instr::Bvs => &["bvs"],
            Instr::Bvc => &["bvc"],
            Instr::Bhi => &["bhi"],
            Instr::Bls => &["bls"],
            Instr::Bge => &["bge"],
            Instr::Blt => &["blt"],
            Instr::Bgt => &["bgt"],
            Instr::Ble => &["ble"],
            Instr::Bal => &["bal"],
        }
    }

    pub fn bits(&self) -> BitVec {
        use Instr::*;
        match &self {
            Lsls => bitvec![u8, Msb0; 0, 0, 0, 0, 0],
            Lsrs => bitvec![u8, Msb0; 0, 0, 0, 0, 1],
            Asrs => bitvec![u8, Msb0; 0, 0, 0, 1, 0],
            Adds => bitvec![u8, Msb0; 0, 0, 0, 1, 1, 0, 0], // ADDS <Rd > , < Rn > , <Rm>
            Adds2 => bitvec![u8, Msb0; 0, 0, 0, 1, 1, 1, 0], // ADDS <Rd > , < Rn> , <#imm3>
            Adds3 => bitvec![u8, Msb0; 0, 0, 1, 1, 0],      // ADDS <Rdn > , #<imm8>
            Subs => bitvec![u8, Msb0; 0, 0, 0, 1, 1, 0, 1], // SUBS <Rd > , < Rn > , <Rm>
            Subs2 => bitvec![u8, Msb0; 0, 0, 0, 1, 1, 1, 1], // SUBS <Rd > , < Rn> , <#imm3>
            Subs3 => bitvec![u8, Msb0; 0, 0, 1, 1, 1],      // SUBS <Rdn > , #<imm8>
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
            Cmp2 => bitvec![u8, Msb0; 0, 0, 1, 0, 1],
            Cmn => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 0, 1, 1],
            Orrs => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 1, 0, 0],
            Muls => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 1, 0, 1],
            Bics => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 1, 1, 0],
            Mvns => bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 1, 1, 1],
            // Load / Store
            Str => bitvec![u8, Msb0; 1, 0, 0, 1, 0],
            Ldr => bitvec![u8, Msb0; 1, 0, 0, 1, 1],
            Ldr2 => bitvec![u8, Msb0; 0, 1, 1, 0, 1],
            Ldr3 => Self::bits(&Movs), // implemented as movs
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

pub type Immediate3 = Immediate<3, false>;
pub type Immediate5 = Immediate<5, false>;
pub type Immediate8 = Immediate<8, false>;
pub type Immediate11 = SignedImmediate<11, false>;

pub type Immediate8S = SignedImmediate<8, false>;

pub type Immediate7W = Immediate<7, true>;
pub type Immediate8W = Immediate<8, true>;

/// List of all possible instructions arguments
#[derive(PartialEq, Debug, Clone)]
pub enum Args {
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
    RtRnImm5(Reg, Reg, Immediate5),
    RtLabel(Reg, String),
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
    #[error("Invalid instr / arg combination")]
    InvalidArg,
}

/// Complete the instruction by replacing labels with their actual address
/// conditional jumps can use 8 bits to encode the distance
fn complete_label_imm8(label: usize, cur_line: usize) -> Result<Immediate8S, CompleteError> {
    let offset = label as i16 - cur_line as i16 - 3;

    let imm = Immediate8S::new(offset).map_err(|_| CompleteError::JumpTooFar {
        label: label.to_string(),
        distance: offset as i32,
    })?;

    Ok(imm)
}

/// Complete the instruction by replacing labels with their actual address
/// Unconditional jumps can use 11 bits to encode the distance
fn complete_label_imm11(label: usize, cur_line: usize) -> Result<Immediate11, CompleteError> {
    let offset = label as i16 - cur_line as i16 - 3;

    let imm = Immediate11::new(offset).map_err(|_| CompleteError::JumpTooFar {
        label: label.to_string(),
        distance: offset as i32,
    })?;

    Ok(imm)
}

impl FullInstr {
    /// Complete the instruction by replacing labels with their actual address
    /// and checking that the jump is not too far away
    pub fn complete(
        &self,
        cur_line: usize,
        rom_labels: &LabelLookup,
        ram_labels: &LabelLookup,
    ) -> Result<FullInstr, CompleteError> {
        let mut copy = self.clone();
        if let Args::Label(ref label) = self.args {
            if let Some(&addr) = rom_labels.get(label) {
                copy.args = match self {
                    FullInstr {
                        instr: Instr::B, ..
                    } => Args::Immediate11(complete_label_imm11(addr, cur_line)?),
                    _ => Args::Immediate8S(complete_label_imm8(addr, cur_line)?),
                }
            } else {
                return Err(CompleteError::LabelNotFound(label.clone()));
            }
        }
        if let FullInstr {
            instr: Instr::Ldr3,
            args: Args::RtLabel(rt, label),
        } = self
        {
            if let Some(&addr) = ram_labels.get(label) {
                // so, this is complicated. We are outputting our own ram
                // we use r7 as the register containing the address of our ram
                copy.args = Args::RdImm8(
                    *rt,
                    Immediate8::new(addr as u16).map_err(|_| CompleteError::JumpTooFar {
                        label: label.clone(),
                        distance: addr as i32,
                    })?,
                );
            } else {
                return Err(CompleteError::LabelNotFound(label.clone()));
            }
        }
        Ok(copy)
    }
}
