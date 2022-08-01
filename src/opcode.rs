/// OpCodes of the Chip-8 Virtual Machine.
///
/// Source: https://en.wikipedia.org/wiki/CHIP-8#Opcode_table
#[derive(Debug, PartialEq)]
pub enum Operation {
    /// Code: 0NNN
    ///
    /// No Operation.
    NoOperation,

    /// Code: 00E0
    ///
    /// Clears the screen.
    ClearDisplay,

    /// Code: 00EE
    ///
    /// Returns from a subroutine.
    SubroutineReturn,

    /// Code: 1NNN
    ///
    /// Jumps to address NNN.
    GotoAddress { nnn: u16 },

    /// Code: 2NNN
    ///
    /// Calls subroutine at NNN.
    SubroutineCall { nnn: u16 },

    /// Code: 3XNN
    ///
    /// Skips the next instruction if VX equals nn.
    /// (Usually the next instruction is a jump to skip a code block);
    EqualityCheck { x: u8, nn: u8 },

    /// Code: 4XNN
    ///
    /// Skips the next instruction if VX does not equal nn.
    /// (Usually the next instruction is a jump to skip a code block);
    InequalityCheck { x: u8, nn: u8 },

    /// Code: 5XY0
    ///
    /// Skips the next instruction if VX equals VY.
    /// (Usually the next instruction is a jump to skip a code block);
    EqualityRegisterCheck { x: u8, y: u8 },

    /// Code: 6XNN
    ///
    /// Sets VX to nn.
    SetRegister { x: u8, nn: u8 },

    /// Code: 7XNN
    ///
    /// Adds nn to VX. (Carry flag is not changed);
    AddRegister { x: u8, nn: u8 },

    /// Code: 8XY0
    ///
    /// Sets VX to the value of VY.
    SetRegisterFromRegister { x: u8, y: u8 },

    /// Code: 8XY1
    ///
    /// Sets VX to VX or VY. (Bitwise OR operation);
    BitwiseOr { x: u8, y: u8 },

    /// Code: 8XY2
    ///
    /// Sets VX to VX and VY. (Bitwise AND operation);
    BitwiseAnd { x: u8, y: u8 },

    /// Code: 8XY3
    ///
    /// Sets VX to VX xor VY.
    BitwiseXor { x: u8, y: u8 },

    /// Code: 8XY4
    ///
    /// Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
    AddValues { x: u8, y: u8 },

    /// Code: 8XY5
    ///
    /// VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
    SubtractValues { x: u8, y: u8 },

    /// Code: 8XY6
    ///
    /// Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
    StoreLeastSignificant { x: u8, y: u8 },

    /// 8XY7
    ///
    /// Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not.
    SubtractValueFromRegister { x: u8, y: u8 },

    /// Code: 8XYE
    ///
    /// Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
    StoreMostSignificant { x: u8, y: u8 },

    /// Code: 9XY0
    ///
    /// Skips the next instruction if VX does not equal VY.
    /// (Usually the next instruction is a jump to skip a code block);
    InequalityRegister { x: u8, y: u8 },

    /// Code: ANNN
    ///
    /// Sets I to the address NNN. 
    SetIndexToAddress { nnn: u16 },

    /// Code: BNNN
    ///
    /// Jumps to the address NNN plus V0.
    GotoAddressWithRegister { nnn: u16 },

    /// Code: CXNN
    ///
    /// Sets VX to the result of a bitwise and operation on a random number
    /// (Typically: 0 to 255) and NN.
    AssignRandomNumber { x: u8, nn: u8 },

    /// Code: DXYN
    ///
    /// Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
    /// Each row of 8 pixels is read as bit-coded starting from memory location I;
    /// I value does not change after the execution of this instruction. As described above,
    /// VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn,
    /// and to 0 if that does not happen
    DrawSprite { x: u8, y: u8, n: u8 },

    /// Code: EX9E
    ///
    /// Skips the next instruction if the key stored in VX is pressed.
    /// (Usually the next instruction is a jump to skip a code block);
    SkipIfKeyPressed { x: u8 },

    /// Code: EXA1
    ///
    /// Skips the next instruction if the key stored in VX is not pressed.
    /// (Usually the next instruction is a jump to skip a code block);
    SkipIfKeyNotPressed { x: u8 },

    /// Code: FX07
    ///
    /// Sets VX to the value of the delay timer.
    GetDelayTimer { x: u8 },

    /// Code: FX0A
    ///
    /// A key press is awaited, and then stored in VX.
    /// (Blocking Operation. All instruction halted until next key event);
    StoreNextKeypress { x: u8 },

    /// Code: FX15
    ///
    /// Sets the delay timer to VX.
    SetDelayTimer { x: u8 },

    /// Code: FX18
    ///
    /// Sets the sound timer to VX.
    SetSoundTimer { x: u8 },

    /// Code: FX1E
    ///
    /// Adds VX to I. VF is not affected.
    AddToIndex { x: u8 },

    /// Code: FX29
    ///
    /// Sets I to the location of the sprite for the character in VX.
    /// Characters 0-F (in hexadecimal) are represented by a 4x5 font.
    SetIndexToSprite { x: u8 },

    /// Code: FX33
    ///
    /// Stores the binary-coded decimal representation of VX,
    /// with the most significant of three digits at the address in I,
    /// the middle digit at I plus 1, and the least significant digit at I plus 2.
    /// (In other words, take the decimal representation of VX,
    /// place the hundreds digit in memory at location in I,
    /// the tens digit at location I+1, and the ones digit at location I+2.);
    StoreBinaryCodedDecimal { x: u8 },

    /// Code: FX55
    ///
    /// Stores from V0 to VX (including VX) in memory, starting at address I.
    /// The offset from I is increased by 1 for each value written, but I itself is left unmodified.
    StoreRegistersInMemory { x: u8 },

    /// Code: FX65
    ///
    /// Fills from V0 to VX (including VX) with values from memory, starting at address I.
    /// The offset from I is increased by 1 for each value read, but I itself is left unmodified.
    SetRegistersFromMemory { x: u8 },
}

#[inline]
fn parse_x(opcode: u16) -> u8 {
    ((opcode & 0x0F00) >> 8) as u8
}

#[inline]
fn parse_x_y(opcode: u16) -> (u8, u8) { (((opcode & 0x0F00) >> 8) as u8, ((opcode & 0x00F0) >> 4) as u8) }

#[inline]
fn parse_x_y_n(opcode: u16) -> (u8, u8, u8) {
    let x = parse_x(opcode);
    let y = (opcode & 0x00F0) >> 4;
    let n = opcode & 0x000F;
    (x, y as u8, n as u8)
}

#[inline]
fn parse_x_nn(opcode: u16) -> (u8, u8) { (((opcode & 0x0F00) >> 8) as u8, (opcode & 0x00FF) as u8) }

#[inline]
fn parse_nnn(opcode: u16) -> u16 { opcode & 0x0FFF }


/// Decode u16 into Chip-8 opcode.
pub fn decode(opcode: u16) -> Result<Operation, ()> {
    match opcode & 0xF000 {
        0x0000 => match opcode & 0x00FF {
            0x0000 => Ok(Operation::NoOperation),
            0x00E0 => Ok(Operation::ClearDisplay),
            0x00EE => Ok(Operation::SubroutineReturn),
            _ => Err(())
        }
        0x1000 => Ok(Operation::GotoAddress { nnn: parse_nnn(opcode) }),
        0x2000 => Ok(Operation::SubroutineCall { nnn: parse_nnn(opcode) }),
        0x3000 => {
            let (x, nn) = parse_x_nn(opcode);
            Ok(Operation::EqualityCheck { x, nn })
        }
        0x4000 => {
            let (x, nn) = parse_x_nn(opcode);
            Ok(Operation::InequalityCheck { x, nn })
        }
        0x5000 => {
            let (x, y) = parse_x_y(opcode);
            Ok(Operation::EqualityRegisterCheck { x, y })
        }
        0x6000 => {
            let (x, nn) = parse_x_nn(opcode);
            Ok(Operation::SetRegister { x, nn })
        }
        0x7000 => {
            let (x, nn) = parse_x_nn(opcode);
            Ok(Operation::AddRegister { x, nn })
        }
        0x8000 => {
            let (x, y) = parse_x_y(opcode);
            match opcode & 0x000F {
                0x0000 => Ok(Operation::SetRegisterFromRegister { x, y }),
                0x0001 => Ok(Operation::BitwiseOr { x, y }),
                0x0002 => Ok(Operation::BitwiseAnd { x, y }),
                0x0003 => Ok(Operation::BitwiseXor { x, y }),
                0x0004 => Ok(Operation::AddValues { x, y }),
                0x0005 => Ok(Operation::SubtractValues { x, y }),
                0x0006 => Ok(Operation::StoreLeastSignificant { x, y }),
                0x0007 => Ok(Operation::SubtractValueFromRegister { x, y }),
                0x000E => Ok(Operation::StoreMostSignificant { x, y }),
                _ => Err(())
            }
        }
        0x9000 => {
            let (x, y) = parse_x_y(opcode);
            Ok(Operation::InequalityRegister { x, y })
        }
        0xA000 => Ok(Operation::SetIndexToAddress { nnn: parse_nnn(opcode) }),
        0xB000 => Ok(Operation::GotoAddressWithRegister { nnn: parse_nnn(opcode) }),
        0xC000 => {
            let (x, nn) = parse_x_nn(opcode);
            Ok(Operation::AssignRandomNumber { x, nn })
        }
        0xD000 => {
            let (x, y, n) = parse_x_y_n(opcode);
            Ok(Operation::DrawSprite { x, y, n })
        }
        0xE000 => {
            let x = parse_x(opcode);
            match opcode & 0x00FF {
                0x009E => Ok(Operation::SkipIfKeyPressed { x }),
                0x00A1 => Ok(Operation::SkipIfKeyNotPressed { x }),
                _ => Err(())
            }
        }
        0xF000 => {
            let x = parse_x(opcode);
            match opcode & 0x00FF {
                0x0007 => Ok(Operation::GetDelayTimer { x }),
                0x000A => Ok(Operation::StoreNextKeypress { x }),
                0x0015 => Ok(Operation::SetDelayTimer { x }),
                0x0018 => Ok(Operation::SetSoundTimer { x }),
                0x001E => Ok(Operation::AddToIndex { x }),
                0x0029 => Ok(Operation::SetIndexToSprite { x }),
                0x0033 => Ok(Operation::StoreBinaryCodedDecimal { x }),
                0x0055 => Ok(Operation::StoreRegistersInMemory { x }),
                0x0065 => Ok(Operation::SetRegistersFromMemory { x }),
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case(0x0000, 0)]
    #[case(0xC100, 1)]
    #[case(0xA400, 4)]
    #[case(0x6A00, 10)]
    #[case(0x1D00, 13)]
    #[case(0xBF00, 15)]
    fn test_parse_x(#[case] opcode: u16, #[case] expected_x: u8) {
        assert_eq!(parse_x(opcode), expected_x);
    }

    #[rstest]
    #[case(0xF3C0, 3, 12)]
    #[case(0x3F00, 15, 0)]
    #[case(0xC320, 3, 2)]
    #[case(0x6910, 9, 1)]
    #[case(0xAFF0, 15, 15)]
    #[case(0x0000, 0, 0)]
    fn test_parse_x_y(#[case] opcode: u16, #[case] expected_x: u8, #[case] expected_y: u8) {
        assert_eq!(parse_x_y(opcode), (expected_x, expected_y));
    }

    #[rstest]
    #[case(0x13BA, 3, 11, 10)]
    #[case(0xFA82, 10, 8, 2)]
    #[case(0x7CA1, 12, 10, 1)]
    #[case(0xEFAF, 15, 10, 15)]
    #[case(0x0113, 1, 1, 3)]
    #[case(0xC543, 5, 4, 3)]
    fn test_parse_x_y_n(
        #[case] opcode: u16,
        #[case] expected_x: u8,
        #[case] expected_y: u8,
        #[case] expected_n: u8,
    ) {
        assert_eq!(parse_x_y(opcode), (expected_x, expected_y));
    }

    #[rstest]
    #[case(0x234F, 3, 79)]
    #[case(0x9AD1, 10, 209)]
    #[case(0xDC10, 12, 16)]
    #[case(0xF00F, 0, 15)]
    #[case(0x43AA, 3, 170)]
    #[case(0xA000, 0, 0)]
    fn test_parse_x_nn(#[case] opcode: u16, #[case] expected_x: u8, #[case] expected_nn: u8) {
        assert_eq!(parse_x_nn(opcode), (expected_x, expected_nn));
    }

    #[rstest]
    #[case(0xCCAA, 3242)]
    #[case(0x2342, 834)]
    #[case(0x23F1, 1009)]
    #[case(0xA1D5, 469)]
    #[case(0x4001, 1)]
    #[case(0x0000, 0)]
    fn test_parse_nnn(#[case] opcode: u16, #[case] expected_nnn: u16) {
        assert_eq!(parse_nnn(opcode), expected_nnn);
    }
}