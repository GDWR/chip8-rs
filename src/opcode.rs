/// OpCodes of the Chip-8 Virtual Machine.
///
/// Source: https://en.wikipedia.org/wiki/CHIP-8#Opcode_table
#[derive(PartialEq)]
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
    GotoAddress { nnn: u8 },

    /// Code: 2NNN
    ///
    /// Calls subroutine at NNN.
    SubroutineCall { nnn: u8 },

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
    SetIndexToAddress { nnn: u8 },

    /// Code: BNNN
    ///
    /// Jumps to the address NNN plus V0.
    GotoAddressWithRegister { nnn: u8 },

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


/// Decode u16 into Chip-8 opcode.
pub fn decode(opcode: u16) -> Result<Operation, ()> {
    match opcode & 0xF000 {
        0x0000 => match opcode & 0x00FF {
            0x0000 => Ok(Operation::NoOperation),
            0x00E0 => Ok(Operation::ClearDisplay),
            0x00EE => Ok(Operation::SubroutineReturn),
            _ => Err(())
        }
        _ => Err(())
    }
}


#[test]
fn decode_basic_opcodes() {
    matches!(decode(0x0000).unwrap(), Operation::NoOperation);
    matches!(decode(0x00E0).unwrap(), Operation::ClearDisplay);
    matches!(decode(0x00EE).unwrap(), Operation::SubroutineReturn);
}