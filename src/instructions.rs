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
pub struct Immediate11(pub u16);

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RdRmImm5(pub Reg, pub Reg, pub Immediate5);

#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Instr {
    Lsls = 0,
    Lsrs = 1,
    Asrs = 2,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Args {
    RdRmImm5(RdRmImm5),
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct FullInstr {
    pub instr: Instr,
    pub args: Args,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ParsedLine {
    Instr(FullInstr),
    Label(String),
    None,
}
