use rstest::*;

use chip8::opcode::{decode, Operation};

#[rstest]
#[case(0x0000, Operation::NoOperation)]
#[case(0x00E0, Operation::ClearDisplay)]
#[case(0x00EE, Operation::SubroutineReturn)]
#[case(0x1000, Operation::GotoAddress { nnn: 0 })]
#[case(0x2000, Operation::SubroutineCall { nnn: 0 })]
#[case(0x3000, Operation::EqualityCheck { x: 0, nn: 0 })]
#[case(0x5000, Operation::EqualityRegisterCheck { x: 0, y: 0 })]
#[case(0x6000, Operation::SetRegister { x: 0, nn: 0 })]
#[case(0x7000, Operation::AddRegister { x: 0, nn: 0 })]
#[case(0x8000, Operation::SetRegisterFromRegister { x: 0, y: 0 })]
#[case(0x8001, Operation::BitwiseOr { x: 0, y: 0 })]
#[case(0x8002, Operation::BitwiseAnd { x: 0, y: 0 })]
#[case(0x8003, Operation::BitwiseXor { x: 0, y: 0 })]
#[case(0x8004, Operation::AddValues { x: 0, y: 0 })]
#[case(0x8005, Operation::SubtractValues { x: 0, y: 0 })]
#[case(0x8006, Operation::StoreLeastSignificant { x: 0, y: 0 })]
#[case(0x8007, Operation::SubtractValueFromRegister { x: 0, y: 0 })]
#[case(0x800E, Operation::StoreMostSignificant { x: 0, y: 0 })]
#[case(0x9000, Operation::InequalityRegister { x: 0, y: 0 })]
#[case(0xA000, Operation::SetIndexToAddress { nnn: 0 })]
#[case(0xB000, Operation::GotoAddressWithRegister { nnn: 0 })]
#[case(0xC000, Operation::AssignRandomNumber { x: 0, nn: 0 })]
#[case(0xD000, Operation::DrawSprite { x: 0, y: 0, n: 0 })]
#[case(0xE09E, Operation::SkipIfKeyPressed { x: 0 })]
#[case(0xE0A1, Operation::SkipIfKeyNotPressed { x: 0 })]
#[case(0xF007, Operation::GetDelayTimer { x: 0 })]
#[case(0xF00A, Operation::StoreNextKeypress { x: 0 })]
#[case(0xF015, Operation::SetDelayTimer { x: 0 })]
#[case(0xF018, Operation::SetSoundTimer { x: 0 })]
#[case(0xF01E, Operation::AddToIndex { x: 0 })]
#[case(0xF029, Operation::SetIndexToSprite { x: 0 })]
#[case(0xF033, Operation::StoreBinaryCodedDecimal { x: 0 })]
#[case(0xF055, Operation::StoreRegistersInMemory { x: 0 })]
#[case(0xF065, Operation::SetRegistersFromMemory { x: 0 })]
fn test_decode(#[case] opcode: u16, #[case] _expected: Operation) {
    let operation = decode(opcode).unwrap();
    assert!(matches!(operation, _expected));
}
