pub mod operand;
pub mod register;

use macros::VMInstruction;
use operand::{Imm16, Imm8};
use register::Register;

#[derive(Debug, PartialEq, Eq, VMInstruction)]
pub enum Instruction {
    #[opcode(0xff)]
    Nop,
    // ---Binary Operators---
    #[opcode(0x1)] //8 bits
    Add {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    #[opcode(0x2)]
    Sub {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    #[opcode(0x3)]
    Mul {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    #[opcode(0x4)]
    And {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    #[opcode(0x5)]
    Or {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    #[opcode(0x6)]
    Xor {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    /// Shift Left
    #[opcode(0x7)]
    Shl {
        dest: Register,
        src: Register,
        shift: Register,
    },
    /// Shift Right Logical
    #[opcode(0x8)]
    Shr {
        dest: Register,
        src: Register,
        shift: Register,
    },
    /// Shift Right Arith
    #[opcode(0x9)]
    ShrA {
        dest: Register,
        src: Register,
        shift: Register,
    },
    // --- Imm ---
    #[opcode(0x13)]
    AddI { dest: Register, src: Imm16 },
    #[opcode(0xc)]
    LoadWord {
        dest: Register,
        src: Register,
        offset: Imm8,
    },
    #[opcode(0xd)]
    StoreWord {
        dest: Register,
        src: Register,
        offset: Imm8,
    },
    // #[opcode(0xe)]
    // LoadByte {
    //     dest: Register,
    //     src: Register,
    //     offset: Imm8,
    // },
    // #[opcode(0xff)]
    // Jal {
    //     dest: Register,
    //     // offset:
    // },
    #[opcode(0x73)]
    Syscall { number: Register },
    // #[opcode(0xff)]
    // Syscall { number: u32 },
    // #[opcode(0x0)]
    // Halt,
}

// pub trait InstructionHandler {
//     fn fetch(&self, memory: u8) -> Instruction;

//     fn decode(&self, opcode: Instruction) -> Result<(), ()>;
// }

#[cfg(test)]
mod test {
    use super::*;
    use register::*;

    #[test]
    fn test_opcode() {
        let op1 = u32::from(&Instruction::Nop) as u8;

        assert_eq!(op1.to_le_bytes(), 0xff_u8.to_le_bytes());
    }

    #[test]
    fn test_encodings() -> Result<(), DecodeError> {
        let ops: Vec<Instruction> = vec![
            Instruction::Add {
                dest: Register::X2,
                src1: Register::X3,
                src2: Register::X4,
            },
            Instruction::LoadWord {
                dest: Register::X4,
                src: Register::X10,
                offset: Imm8(240),
            },
            Instruction::StoreWord {
                dest: Register::X4,
                src: Register::X9,
                offset: Imm8(255),
            },
            Instruction::Nop,
            Instruction::Syscall {
                number: Register::X7,
            },
        ];
        // if (ins & 0x8000) == 0 {

        // println!("{}", 0xff);
        // println!("{}", 0xfff);
        // let dest_val: u8 = unsafe { transmute(Register::X2) };
        // let src1_val: u8 = unsafe { transmute(Register::X3) };
        // let src2_val: u8 = unsafe { transmute(Register::X4) };

        // let encoded: u32 = (1u8 as u32)
        //     | ((dest_val as u32) << 8)
        //     | ((src1_val as u32) << 16)
        //     | ((src2_val as u32) << 24);
        // println!("MANUAL ENCODING u32: {:?}", encoded);

        // let dest = unsafe { transmute::<u8, Register>((encoded >> 8) as u8) };
        // let src1 = unsafe { transmute::<u8, Register>((encoded >> 16) as u8) };
        // let src2 = unsafe { transmute::<u8, Register>((encoded >> 24) as u8) };

        // let result = Instruction::Add { dest, src1, src2 };
        // // let result = Instruction::try_from(result).unwrap();

        // println!("MANUAL ENCODING Instruction: {:?}", result);

        // let a1 = 10 & 11;
        // let a2 = 11 & 10;
        // println!("A1: {a1}");
        // println!("A2: {a2}");

        let encoded: Vec<u32> = ops.iter().map(|x| x.into()).collect();
        for (i, (l, r)) in ops.iter().zip(encoded.iter()).enumerate() {
            println!("{i} l: {:?}, r: {1}", l, r);
            let decoded = Instruction::try_from(*r)?;
            println!("Decoded: {:?}", decoded);
            assert_eq!(*l, decoded);
        }
        Ok(())
    }
}
