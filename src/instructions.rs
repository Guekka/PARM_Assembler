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
        if value == Reg::R0 as u8 {
            Ok(Reg::R0)
        } else if value == Reg::R1 as u8 {
            Ok(Reg::R1)
        } else if value == Reg::R2 as u8 {
            Ok(Reg::R2)
        } else if value == Reg::R3 as u8 {
            Ok(Reg::R3)
        } else if value == Reg::R4 as u8 {
            Ok(Reg::R4)
        } else if value == Reg::R5 as u8 {
            Ok(Reg::R5)
        } else if value == Reg::R6 as u8 {
            Ok(Reg::R6)
        } else if value == Reg::R7 as u8 {
            Ok(Reg::R7)
        } else if value == Reg::PC as u8 {
            Ok(Reg::PC)
        } else if value == Reg::SP as u8 {
            Ok(Reg::SP)
        } else {
            Err(())
        }
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
    None,
}
