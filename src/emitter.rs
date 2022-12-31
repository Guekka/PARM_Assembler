#[allow(clippy::unusual_byte_groupings)]

pub trait ToBinary {
    fn to_binary(&self) -> BitVec<u8, Msb0>;
}
use crate::instructions::*;

use bitvec::prelude::*;

impl ToBinary for Reg {
    fn to_binary(&self) -> BitVec<u8, Msb0> {
        let val = *self as u8;
        let mut bits = BitVec::<u8, Msb0>::new();
        bits.resize(3, false);
        bits.store(val);
        bits
    }
}

impl ToBinary for Immediate5 {
    fn to_binary(&self) -> BitVec<u8, Msb0> {
        let val = self.0;
        val.view_bits::<Msb0>().split_at(3).1.to_owned()
    }
}

impl ToBinary for RdRmImm5 {
    fn to_binary(&self) -> BitVec<u8, Msb0> {
        let mut bits = BitVec::<u8, Msb0>::new();
        bits.extend_from_bitslice(&self.2.to_binary());
        bits.extend_from_bitslice(&self.0.to_binary());
        bits.extend_from_bitslice(&self.1.to_binary());
        bits
    }
}

fn binary_bit_count(instr: &Instr) -> u8 {
    match instr {
        Instr::Lsls => 5,
        Instr::Lsrs => 5,
        Instr::Asrs => 5,
        Instr::B => 8,
        Instr::Beq => 8,
        Instr::Bne => 8,
        Instr::Bcs => 8,
        Instr::Bcc => 8,
        Instr::Bmi => 8,
        Instr::Bpl => 8,
        Instr::Bvs => 8,
        Instr::Bvc => 8,
        Instr::Bhi => 8,
        Instr::Bls => 8,
        Instr::Bge => 8,
        Instr::Blt => 8,
        Instr::Bgt => 8,
        Instr::Ble => 8,
    }
}

impl ToBinary for Instr {
    fn to_binary(&self) -> BitVec<u8, Msb0> {
        let val = *self as u8;
        let bit_count = binary_bit_count(self) as usize;
        let mut bits = BitVec::<u8, Msb0>::with_capacity(bit_count);
        bits.resize(bit_count, false);
        bits.store(val);
        bits
    }
}

impl ToBinary for FullInstr {
    fn to_binary(&self) -> BitVec<u8, Msb0> {
        let mut bits = self.instr.to_binary();
        match self.args {
            Args::RdRmImm5(args) => {
                bits.extend_from_bitslice(&args.to_binary());
            }
            Args::Immediate8(args) => {
                bits.extend_from_bitslice(args.0.view_bits::<Msb0>());
            }
            Args::Label(_) => panic!("Label not resolved"),
        }
        bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reg_to_binary() {
        let reg = Reg::R3;
        let expected = bits![0, 1, 1];
        assert_eq!(reg.to_binary(), expected);
    }

    #[test]
    fn imm5_to_binary() {
        let imm5 = Immediate5(4);
        let expected = bits![0, 0, 1, 0, 0];
        assert_eq!(imm5.to_binary(), expected);
    }

    #[test]
    fn rd_rm_imm5_to_binary() {
        let args = RdRmImm5(Reg::R3, Reg::R4, Immediate5(7));
        let expected = bits![0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0];
        assert_eq!(args.to_binary(), expected);
    }

    #[test]
    fn instr_to_binary() {
        let instr = Instr::Lsrs;
        let expected = bits![0, 0, 0, 0, 1];
        assert_eq!(instr.to_binary(), expected);
    }

    #[test]
    fn lsls() {
        let instr = FullInstr {
            instr: Instr::Lsls,
            args: Args::RdRmImm5(RdRmImm5(Reg::R3, Reg::R4, Immediate5(7))),
        };
        let expected = bits![
            0, 0, 0, 0, 0, // Lsls
            0, 0, 1, 1, 1, // Imm5
            0, 1, 1, // Rm
            1, 0, 0, // Rd
        ];
        assert_eq!(instr.to_binary(), expected);
    }

    #[test]
    fn branch() {
        let instr = FullInstr {
            instr: Instr::B,
            args: Args::Immediate8(Immediate8(0b0000_0010)),
        };
        let expected = bits![
            1, 1, 0, 1, 1, 1, 1, 0, // B
            0, 0, 0, 0, 0, 0, 1, 0 // Imm8
        ];
        assert_eq!(instr.to_binary(), expected);
    }
}
