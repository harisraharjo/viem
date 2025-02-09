use std::fmt::Display;

use crate::{
    operand::{Immediate14, Immediate19},
    register::Register,
};
use macros::VMInstruction;

#[derive(Debug, PartialEq, Eq, VMInstruction)]
// #[repr(u32)]
pub enum Instruction {
    // ---Binary Operators---
    #[isa(0x1, 5, 5, 5)]
    Add {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    #[isa(0x2, 5, 5, 5)]
    Sub {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    #[isa(0x3, 5, 5, 5)]
    Mul {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    #[isa(0x4, 5, 5, 5)]
    And {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    #[isa(0x5, 5, 5, 5)]
    Or {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    #[isa(0x6, 5, 5, 5)]
    Xor {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    /// Shift Left
    #[isa(0x7, 5, 5, 5)]
    Shl {
        dest: Register,
        src: Register,
        shift: Register,
    },
    /// Shift Right Logical
    #[isa(0x8, 5, 5, 5)]
    Shr {
        dest: Register,
        src: Register,
        shift: Register,
    },
    /// Shift Right Arith
    #[isa(0x9, 5, 5, 5)]
    ShrA {
        dest: Register,
        src: Register,
        shift: Register,
    },
    // --- Imm ---
    /// Add Immediate
    #[isa(0x13, 5, 5, 14)]
    AddI {
        dest: Register,
        src: Register,
        value: Immediate14,
    },
    /// Load Upper Immediate.
    #[isa(0x14, 5, 19)]
    Lui { dest: Register, value: Immediate19 },
    /// Load Word
    #[isa(0xc, 5, 5, 14)]
    Lw {
        dest: Register,
        src: Register,
        offset: Immediate14,
    },
    /// Store Word
    #[isa(0xd, 5, 5, 14)]
    Sw {
        dest: Register,
        src: Register,
        offset: Immediate14,
    },
    // #[isa(0xe,5,5,5)]
    // LoadByte {
    //     dest: Register,
    //     src: Register,
    //     offset: u8,
    // },
    // #[isa(0xff,5,5,5)]
    // Jal {
    //     dest: Register,
    //     // offset:
    // },
    // TODO: Exit, halt, shutdown
    // pub const SIGHALT: u8 = 0xf;
    // #[isa(0X5D,5,5,5)]
    #[isa(0x73, 5, 5, 5)]
    Syscall {
        src1: Register,
        src2: Register,
        src3: Register,
    },
    // Pseudo
    /// Load Immediate
    #[isa(0xff, 5, 19)]
    Li { dest: Register, value: Immediate19 },
    // #[isa(0xff,5,5,5)]
    // Syscall { number: u32 },
    // #[isa(0x0,5,5,5)]
    // Halt,
}
// TODO: Add mnemonic. ("add", "addi", etc)

pub trait Codec {
    fn decode(src: u32, bit_accumulation: u32, bit_mask: u32) -> Self
    where
        Self: core::convert::From<u32>,
    {
        ((src >> bit_accumulation) & bit_mask).into()
    }

    fn encode(&self, bit_mask: u32, bit_accumulation: u32) -> u32
    where
        for<'a> &'a Self: std::ops::BitAnd<u32, Output = u32> + std::ops::Shl<u32, Output = u32>,
    {
        (self & bit_mask) << bit_accumulation
    }
}

#[cfg(test)]
mod test {
    use crate::{
        instruction::{DecodeError, Instruction},
        operand::{Immediate, Immediate14, Immediate19},
        register::Register,
    };

    #[test]
    fn t_opcode() {
        let op1 = u32::from(&Instruction::Li {
            dest: Register::Zero,
            value: Immediate19::new(150),
        }) as u8;

        assert_eq!(op1.to_le_bytes(), 0xff_u8.to_le_bytes());
    }

    #[test]
    fn t_encode_decode() -> Result<(), DecodeError> {
        let ins: Vec<Instruction> = vec![
            Instruction::Add {
                dest: Register::A0,
                src1: Register::A1,
                src2: Register::A2,
            },
            Instruction::Li {
                dest: Register::T0,
                value: Immediate19::new(150),
            },
            Instruction::Lui {
                dest: Register::T0,
                value: Immediate19::new(150),
            },
            Instruction::AddI {
                dest: Register::A0,
                src: Register::A1,
                value: Immediate14::new(-31),
            },
            // Instruction::LoadWord {
            //     dest: Register::T2,
            //     src: Register::T3,
            //     offset: Immediate::new(11),
            // },
            // Instruction::StoreWord {
            //     dest: Register::T2,
            //     src: Register::T3,
            //     offset: Immediate::new(11),
            // },
            Instruction::Syscall {
                src1: Register::A1,
                src2: Register::A2,
                src3: Register::A3,
            },
        ];

        let encoded: Vec<u32> = ins.iter().map(|x| x.into()).collect();
        for (l, r) in ins.iter().zip(encoded.iter()) {
            println!("instruction: {:?}, memory representation: {1}", l, r);
            let decoded = Instruction::try_from(*r)?;
            // println!("Decoded: {:?}", decoded);
            assert_eq!(*l, decoded);
        }
        Ok(())
    }
}
