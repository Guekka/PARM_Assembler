#[derive(PartialEq, Debug)]
pub enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    SP,
}
#[derive(PartialEq, Debug)]
pub struct Immediate8(pub u8);

#[derive(PartialEq, Debug)]
pub struct Immediate16(pub u16);

impl TryFrom<u8> for Reg {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Reg::R0),
            1 => Ok(Reg::R1),
            2 => Ok(Reg::R2),
            3 => Ok(Reg::R3),
            4 => Ok(Reg::R4),
            5 => Ok(Reg::R5),
            6 => Ok(Reg::R6),
            7 => Ok(Reg::R7),
            8 => Ok(Reg::SP),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct RdRmImm5(pub Reg, pub Reg, pub Immediate8);

#[derive(PartialEq, Debug)]
pub enum Instr {
    Lsls(RdRmImm5),
}
