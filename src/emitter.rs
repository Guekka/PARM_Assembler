#![allow(clippy::unusual_byte_groupings)]

use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::prelude::AsBits;

use crate::instructions::*;

pub trait ToBinary {
    fn to_binary(&self) -> BitVec;
}

impl ToBinary for Reg {
    fn to_binary(&self) -> BitVec {
        let val = *self as u8;
        let mut bits = BitVec::new();
        bits.resize(3, false);
        bits.store_be(val);
        bits
    }
}

impl<const N: u8, const WIDE: bool> ToBinary for Immediate<N, WIDE> {
    fn to_binary(&self) -> BitVec {
        let mut bits = BitVec::new();
        bits.resize(N as usize, false);
        bits.store_be(self.0);
        bits
    }
}

impl<const N: u8, const WIDE: bool> ToBinary for SignedImmediate<N, WIDE> {
    fn to_binary(&self) -> BitVec {
        let mut bits = BitVec::new();
        bits.resize(N as usize, false);
        bits.store_be(self.0);
        bits
    }
}

impl ToBinary for Instr {
    fn to_binary(&self) -> BitVec {
        self.bits()
    }
}

impl ToBinary for Args {
    fn to_binary(&self) -> BitVec {
        // Each argument set has a different order for the bits.
        // This returns the argument in the correct order.
        let order: Vec<&dyn ToBinary> = match &self {
            Args::RdRmImm5(ref rd, ref rm, ref imm5) => vec![imm5, rm, rd],
            Args::RdRnImm3(rd, rn, imm3) => vec![imm3, rn, rd],
            Args::Label(_) => panic!("Label not resolved"),
            Args::RdRnRm(rd, rn, rm) => vec![rm, rn, rd],
            Args::RdImm8(rd, imm8) => vec![rd, imm8],
            Args::Immediate7W(imm7w) => vec![imm7w],
            Args::TwoRegs(r1, r2) => vec![r2, r1],
            Args::RdRnImm0(rd, rn) => vec![rn, rd],
            Args::Immediate11(imm11) => vec![imm11],
            Args::RtSpImm8W(rt, imm8w) => vec![rt, imm8w],
            Args::Immediate8S(imm8s) => vec![imm8s],
        };
        order
            .into_iter()
            .map(|x| x.to_binary())
            .fold(BitVec::new(), |mut acc, x| {
                acc.extend(x);
                acc
            })
    }
}

impl ToBinary for FullInstr {
    fn to_binary(&self) -> BitVec {
        let mut bits = self.instr.to_binary();
        bits.extend_from_bitslice(&self.args.to_binary());
        bits
    }
}

impl ToBinary for LiteralPool {
    fn to_binary(&self) -> BitVec {
        self.data.iter().fold(BitVec::new(), |mut bits, str| {
            bits.extend_from_bitslice(str.as_bits::<Msb0>());
            bits
        })
    }
}

#[cfg(test)]
mod tests {
    use bitvec::bits;
    use bitvec::order::Msb0;

    use crate::instructions::Args::RdRmImm5;
    use crate::instructions::Immediate11;

    use super::*;

    #[test]
    fn reg_to_binary() {
        let reg = Reg::R3;
        let expected = bits![u8, Msb0; 0, 1, 1];
        assert_eq!(reg.to_binary(), expected);
    }

    #[test]
    fn imm5_to_binary() {
        let imm5 = Immediate5::new(4).unwrap();
        let expected = bits![u8, Msb0; 0, 0, 1, 0, 0];
        assert_eq!(imm5.to_binary(), expected);
    }

    #[test]
    fn rd_rm_imm5_to_binary() {
        let args = RdRmImm5(Reg::R3, Reg::R4, Immediate5::new(7).unwrap());
        let expected = bits![u8, Msb0; 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1];
        assert_eq!(args.to_binary(), expected);
    }

    #[test]
    fn instr_to_binary() {
        let instr = Instr::Lsrs;
        let expected = bits![u8, Msb0; 0, 0, 0, 0, 1];
        assert_eq!(instr.to_binary(), expected);
    }

    #[test]
    fn addsp() {
        let instr = FullInstr {
            instr: Instr::AddSp,
            args: Args::Immediate7W(Immediate7W::new(4).unwrap()),
        };
        let expected = bits![u8, Msb0; 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(instr.to_binary(), expected);
    }

    #[test]
    fn lsls() {
        let instr = FullInstr {
            instr: Instr::Lsls,
            args: Args::RdRmImm5(Reg::R3, Reg::R4, Immediate5::new(7).unwrap()),
        };
        let expected = bits![
            u8, Msb0;
            0, 0, 0, 0, 0, // Lsls
            0, 0, 1, 1, 1, // Imm5
            1, 0, 0, // Rm
            0, 1, 1, // Rd
        ];
        assert_eq!(instr.to_binary(), expected);
    }

    #[test]
    fn branch() {
        let instr = FullInstr {
            instr: Instr::B,
            args: Args::Immediate11(Immediate11::new(0b0000_0000_010).unwrap()),
        };
        let expected = bits![
            u8, Msb0;
            1, 1, 1, 0, 0, // B
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0 // Imm11
        ];
        assert_eq!(instr.to_binary(), expected);
    }

    #[test]
    fn ldr() {
        let input = FullInstr {
            instr: Instr::Ldr,
            args: Args::RtSpImm8W(Reg::R2, Immediate8W::new(4).unwrap()),
        };
        let expected = bits![
            u8, Msb0;
            1, 0, 0, 1, 1, // Ldr
            0, 1, 0, // Rt
            0, 0, 0, 0, 0, 0, 0, 1, // Imm8
        ];
        assert_eq!(input.to_binary(), expected);
    }
}
