use std::collections::HashMap;

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

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Immediate3(pub u8);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Immediate5(pub u8);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Immediate8(pub u8);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Immediate11(pub u16);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RdRmImm5(pub Reg, pub Reg, pub Immediate5);

#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Instr {
    Lsls = 0,
    Lsrs = 1,
    Asrs = 2,
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
    B = 0b1101_1110,
}

impl Instr {
    pub const fn text_instruction(&self) -> &'static str {
        match self {
            Instr::Lsls => "lsls",
            Instr::Lsrs => "lsrs",
            Instr::Asrs => "asrs",
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
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Args {
    RdRmImm5(RdRmImm5),
    Label(String),
    Immediate8(Immediate8),
}

#[derive(PartialEq, Debug, Clone)]
pub struct FullInstr {
    pub instr: Instr,
    pub args: Args,
}

pub type LabelLookup = HashMap<String, usize>;

impl FullInstr {
    pub fn complete(&self, labels: &LabelLookup) -> Result<FullInstr, ()> {
        let mut copy = self.clone();
        if let Args::Label(ref label) = self.args {
            if let Some(&addr) = labels.get(label) {
                copy.args = Args::Immediate8(Immediate8(addr as u8));
            } else {
                return Err(());
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
