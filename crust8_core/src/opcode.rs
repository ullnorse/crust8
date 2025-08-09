use crate::error::EmulatorError;

#[derive(Debug)]
pub enum Opcode {
    ClearScreen,
    ReturnSubroutine,
    Jump(u16),
    Call(u16),
    SkipIfEqByte { x: usize, byte: u8 },
    SkipIfNeqByte { x: usize, byte: u8 },
    SkipIfEqReg { x: usize, y: usize },
    SetReg { x: usize, byte: u8 },
    AddByteToReg { x: usize, byte: u8 },
    SetRegToReg { x: usize, y: usize },
    OrReg { x: usize, y: usize },
    AndReg { x: usize, y: usize },
    XorReg { x: usize, y: usize },
    AddRegToReg { x: usize, y: usize },
    SubRegFromReg { x: usize, y: usize },
    ShrReg { x: usize },
    SubnRegFromReg { x: usize, y: usize },
    ShlReg { x: usize },
    SkipIfNeqReg { x: usize, y: usize },
    SetI(u16),
    JumpV0(u16),
    RndAndByte { x: usize, byte: u8 },
    DrawSprite { x: usize, y: usize, n: usize },
    SkipIfKeyPressed { x: usize },
    SkipIfKeyNotPressed { x: usize },
    SetRegToDelayTimer { x: usize },
    WaitKeyPress { x: usize },
    SetDelayTimer { x: usize },
    SetSoundTimer { x: usize },
    AddRegToI { x: usize },
    SetIToSpriteAddr { x: usize },
    StoreBCD { x: usize },
    StoreRegs { x: usize },
    LoadRegs { x: usize },
}

impl TryFrom<u16> for Opcode {
    type Error = EmulatorError;

    fn try_from(op: u16) -> Result<Self, Self::Error> {
        let nibbles = (
            (op & 0xF000) >> 12,
            (op & 0x0F00) >> 8,
            (op & 0x00F0) >> 4,
            (op & 0x000F),
        );

        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;
        let byte = (op & 0x00FF) as u8;
        let addr = op & 0x0FFF;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => Ok(Opcode::ClearScreen),
            (0x0, 0x0, 0xE, 0xE) => Ok(Opcode::ReturnSubroutine),
            (0x1, _, _, _) => Ok(Opcode::Jump(addr)),
            (0x2, _, _, _) => Ok(Opcode::Call(addr)),
            (0x3, _, _, _) => Ok(Opcode::SkipIfEqByte { x, byte }),
            (0x4, _, _, _) => Ok(Opcode::SkipIfNeqByte { x, byte }),
            (0x5, _, _, 0x0) => Ok(Opcode::SkipIfEqReg { x, y }),
            (0x6, _, _, _) => Ok(Opcode::SetReg { x, byte }),
            (0x7, _, _, _) => Ok(Opcode::AddByteToReg { x, byte }),
            (0x8, _, _, 0x0) => Ok(Opcode::SetRegToReg { x, y }),
            (0x8, _, _, 0x1) => Ok(Opcode::OrReg { x, y }),
            (0x8, _, _, 0x2) => Ok(Opcode::AndReg { x, y }),
            (0x8, _, _, 0x3) => Ok(Opcode::XorReg { x, y }),
            (0x8, _, _, 0x4) => Ok(Opcode::AddRegToReg { x, y }),
            (0x8, _, _, 0x5) => Ok(Opcode::SubRegFromReg { x, y }),
            (0x8, _, _, 0x6) => Ok(Opcode::ShrReg { x }),
            (0x8, _, _, 0x7) => Ok(Opcode::SubnRegFromReg { x, y }),
            (0x8, _, _, 0xE) => Ok(Opcode::ShlReg { x }),
            (0x9, _, _, 0x0) => Ok(Opcode::SkipIfNeqReg { x, y }),
            (0xA, _, _, _) => Ok(Opcode::SetI(addr)),
            (0xB, _, _, _) => Ok(Opcode::JumpV0(addr)),
            (0xC, _, _, _) => Ok(Opcode::RndAndByte { x, byte }),
            (0xD, _, _, _) => Ok(Opcode::DrawSprite { x, y, n }),
            (0xE, _, 0x9, 0xE) => Ok(Opcode::SkipIfKeyPressed { x }),
            (0xE, _, 0xA, 0x1) => Ok(Opcode::SkipIfKeyNotPressed { x }),
            (0xF, _, 0x0, 0x7) => Ok(Opcode::SetRegToDelayTimer { x }),
            (0xF, _, 0x0, 0xA) => Ok(Opcode::WaitKeyPress { x }),
            (0xF, _, 0x1, 0x5) => Ok(Opcode::SetDelayTimer { x }),
            (0xF, _, 0x1, 0x8) => Ok(Opcode::SetSoundTimer { x }),
            (0xF, _, 0x1, 0xE) => Ok(Opcode::AddRegToI { x }),
            (0xF, _, 0x2, 0x9) => Ok(Opcode::SetIToSpriteAddr { x }),
            (0xF, _, 0x3, 0x3) => Ok(Opcode::StoreBCD { x }),
            (0xF, _, 0x5, 0x5) => Ok(Opcode::StoreRegs { x }),
            (0xF, _, 0x6, 0x5) => Ok(Opcode::LoadRegs { x }),
            _ => Err(EmulatorError::UnknownOpcode(op)),
        }
    }
}
