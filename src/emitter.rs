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
        bits.store_be(val);
        bits
    }
}

impl<const N: u8, const WIDE: bool> ToBinary for Immediate<N, WIDE> {
    fn to_binary(&self) -> BitVec<u8, Msb0> {
        let mut bits = BitVec::<u8, Msb0>::new();
        bits.resize(N as usize, false);
        bits.store_be(self.0);
        bits
    }
}

impl<const N: u8, const WIDE: bool> ToBinary for SignedImmediate<N, WIDE> {
    fn to_binary(&self) -> BitVec<u8, Msb0> {
        let mut bits = BitVec::<u8, Msb0>::new();
        bits.resize(N as usize, false);
        bits.store_be(self.0);
        bits
    }
}

impl ToBinary for RdRmImm5 {
    fn to_binary(&self) -> BitVec<u8, Msb0> {
        let mut bits = BitVec::<u8, Msb0>::new();
        bits.extend_from_bitslice(&self.2.to_binary());
        bits.extend_from_bitslice(&self.1.to_binary());
        bits.extend_from_bitslice(&self.0.to_binary());
        bits
    }
}

fn binary_bit_count(instr: &Instr) -> u8 {
    match instr {
        Instr::Lsls => 5,
        Instr::Lsrs => 5,
        Instr::Asrs => 5,
        Instr::Adds => 7,
        Instr::Subs => 7,
        Instr::Adds2 => 7,
        Instr::Subs2 => 7,
        Instr::Movs => 3,
        Instr::Str => 5,
        Instr::Ldr => 5,
        Instr::AddSp => 9,
        Instr::SubSp => 9,
        Instr::Ands => 10,
        Instr::Eors => 10,
        Instr::Lsls2 => 10,
        Instr::Lsrs2 => 10,
        Instr::Asrs2 => 10,
        Instr::Adcs => 10,
        Instr::Sbcs => 10,
        Instr::Rors => 10,
        Instr::Tst => 10,
        Instr::Cmn => 10,
        Instr::Orrs => 10,
        Instr::Muls => 10,
        Instr::Bics => 10,
        Instr::Mvns => 10,
        Instr::Cmp => 10,
        Instr::Rsbs => 10,
        Instr::B => 5,
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
        Instr::Bal => 8,
    }
}

impl ToBinary for Instr {
    fn to_binary(&self) -> BitVec<u8, Msb0> {
        let val = *self as u16;
        let bit_count = binary_bit_count(self) as usize;
        let mut bits = BitVec::<u8, Msb0>::with_capacity(bit_count);
        bits.resize(bit_count, false);
        bits.store_be(val);
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
                bits.extend_from_bitslice(&args.to_binary());
            }
            Args::RdRnImm3(args) => {
                bits.extend_from_bitslice(&args.2.to_binary());
                bits.extend_from_bitslice(&args.1.to_binary());
                bits.extend_from_bitslice(&args.0.to_binary());
            }
            Args::Label(_) => panic!("Label not resolved"),
            Args::RdRnRm(args) => {
                bits.extend_from_bitslice(&args.2.to_binary());
                bits.extend_from_bitslice(&args.1.to_binary());
                bits.extend_from_bitslice(&args.0.to_binary());
            }
            Args::RdImm8(args) => {
                bits.extend_from_bitslice(&args.0.to_binary());
                bits.extend_from_bitslice(&args.1.to_binary());
            }
            Args::Immediate7W(args) => {
                bits.extend_from_bitslice(&args.to_binary());
            }
            Args::TwoRegs(args) => {
                bits.extend_from_bitslice(&args.1.to_binary());
                bits.extend_from_bitslice(&args.0.to_binary());
            }
            Args::RdRnImm0(args) => {
                bits.extend_from_bitslice(&args.1.to_binary());
                bits.extend_from_bitslice(&args.0.to_binary());
            }
            Args::Immediate11(args) => {
                bits.extend_from_bitslice(&args.to_binary());
            }
            Args::RtSpImm8W(args) => {
                bits.extend_from_bitslice(&args.0.to_binary());
                bits.extend_from_bitslice(&args.1.to_binary());
            }
            Args::Immediate8S(args) => {
                bits.extend_from_bitslice(&args.to_binary());
            }
        }
        bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::Immediate11;

    #[test]
    fn reg_to_binary() {
        let reg = Reg::R3;
        let expected = bits![0, 1, 1];
        assert_eq!(reg.to_binary(), expected);
    }

    #[test]
    fn imm5_to_binary() {
        let imm5 = Immediate5::new(4).unwrap();
        let expected = bits![0, 0, 1, 0, 0];
        assert_eq!(imm5.to_binary(), expected);
    }

    #[test]
    fn rd_rm_imm5_to_binary() {
        let args = RdRmImm5(Reg::R3, Reg::R4, Immediate5::new(7).unwrap());
        let expected = bits![0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1];
        assert_eq!(args.to_binary(), expected);
    }

    #[test]
    fn instr_to_binary() {
        let instr = Instr::Lsrs;
        let expected = bits![0, 0, 0, 0, 1];
        assert_eq!(instr.to_binary(), expected);
    }

    #[test]
    fn addsp() {
        let instr = FullInstr {
            instr: Instr::AddSp,
            args: Args::Immediate7W(Immediate7W::new(4).unwrap()),
        };
        let expected = bits![1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(instr.to_binary(), expected);
    }

    #[test]
    fn lsls() {
        let instr = FullInstr {
            instr: Instr::Lsls,
            args: Args::RdRmImm5(RdRmImm5(Reg::R3, Reg::R4, Immediate5::new(7).unwrap())),
        };
        let expected = bits![
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
            1, 1, 1, 0, 0, // B
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0 // Imm11
        ];
        assert_eq!(instr.to_binary(), expected);
    }

    #[test]
    fn ldr() {
        let input = FullInstr {
            instr: Instr::Ldr,
            args: Args::RtSpImm8W(RtSpImm8W(Reg::R2, Immediate8W::new(4).unwrap())),
        };
        let expected = bits![
            1, 0, 0, 1, 1, // Ldr
            0, 1, 0, // Rt
            0, 0, 0, 0, 0, 0, 0, 1, // Imm8
        ];
        assert_eq!(input.to_binary(), expected);
    }
}
