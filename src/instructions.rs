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
pub struct Immediate5(pub u8);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Immediate8(pub u8);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Immediate7W(pub u8);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Immediate8W(pub u8);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Immediate3(pub u8);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Immediate11(pub u16);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RdRmImm5(pub Reg, pub Reg, pub Immediate5);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RdRnRm(pub Reg, pub Reg, pub Reg);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RdRnImm3(pub Reg, pub Reg, pub Immediate3);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RdImm8(pub Reg, pub Immediate8);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RnRm(pub Reg, pub Reg);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RdRnImm0(pub Reg, pub Reg);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RtSpImm8W(pub Reg, pub Immediate8W);

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
    Cmp = 0b010000_1010,
    Rsbs = 0b010000_1001,
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
            Instr::Cmp => "cmp",
            Instr::Rsbs => "rsbs",
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

#[derive(PartialEq, Debug, Clone)]
pub enum Args {
    RdRmImm5(RdRmImm5),
    Label(String),
    Immediate8(Immediate8),
    RdRnRm(RdRnRm),
    RdRnImm3(RdRnImm3),
    RdImm8(RdImm8),
    Immediate7W(Immediate7W),
    Immediate11(Immediate11),
    RnRm(RnRm),
    RdRnImm0(RdRnImm0),
    RtSpImm8W(RtSpImm8W),
}

#[derive(PartialEq, Debug, Clone)]
pub struct FullInstr {
    pub instr: Instr,
    pub args: Args,
}

pub type LabelLookup = HashMap<String, usize>;

fn complete_bcond(label: usize, cur_line: usize) -> Result<Args, ()> {
    let offset = label as i16 - cur_line as i16 - 3;

    i8::try_from(offset)
        .map(|offset| Args::Immediate8(Immediate8(offset as u8)))
        .map_err(|_| ())
}

fn complete_buncond(label: usize, cur_line: usize) -> Result<Args, ()> {
    let offset = label as i16 - cur_line as i16 - 3;

    if !(-1024..=1023).contains(&offset) {
        return Err(());
    }
    Ok(Args::Immediate11(Immediate11(offset as u16)))
}

impl FullInstr {
    pub fn complete(&self, cur_line: usize, labels: &LabelLookup) -> Result<FullInstr, ()> {
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
