use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

use sdl2::event::EventPollIterator;
use sdl2::libc::{sleep, time};

use crate::input_output::InputOutput;
use crate::opcode::{decode, Operation};

pub struct System {
    pub draw_flag: bool,
    ops: u64,
    program_counter: u16,
    index: u16,
    memory: [u8; 4096],
    register: [u8; 16],
    stack: [u16; 8],
    stack_pointer: u8,
    io: InputOutput,
}

impl Default for System {
    fn default() -> Self {
        System {
            draw_flag: false,
            ops: 0,
            program_counter: 0x200,
            index: 0,
            memory: [0; 4096],
            register: [0; 16],
            stack: [0; 8],
            stack_pointer: 0,
            io: InputOutput::default(),
        }
    }
}

impl System {
    pub fn load_rom(&mut self, data: Vec<u8>) {
        for (i, d) in data.iter().enumerate() {
            self.memory[i + 0x200] = *d;
        }
    }

    pub fn load_rom_from_file<P: AsRef<Path>>(&mut self, filepath: P) {
        let data = fs::read(filepath).unwrap();
        self.load_rom(data)
    }

    pub fn iter_events(&mut self) -> EventPollIterator {
        self.io.iter_events()
    }

    pub fn step(&mut self) {
        let l = (self.memory[self.program_counter as usize] as u16) << 8;
        let r = (self.memory[self.program_counter as usize + 1] as u16);
        let opcode = l | r;

        match decode(opcode).unwrap() {
            Operation::NoOperation => self.program_counter += 2,
            Operation::ClearDisplay => {
                self.io.clear();
                self.program_counter += 2;
                self.draw_flag = true;
            }
            Operation::SubroutineReturn => {
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer as usize];
            }
            Operation::GotoAddress { nnn } => {
                self.program_counter = nnn;
            }
            Operation::SubroutineCall { nnn } => {
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = nnn;
            }
            Operation::EqualityCheck { x, nn } => {
                if self.register[x as usize] == nn {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            Operation::InequalityCheck { x, nn } => {
                if self.register[x as usize] != nn {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            Operation::EqualityRegisterCheck { x, y } => {
                if self.register[x as usize] == self.register[y as usize] {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            Operation::SetRegister { x, nn } => {
                self.register[x as usize] = nn;
                self.program_counter += 2;
            }
            Operation::AddRegister { x, nn } => {
                let mut value = self.register[x as usize] as u16;
                value += nn as u16;
                self.register[x as usize] = value as u8;
                self.program_counter += 2;
            }
            Operation::SetRegisterFromRegister { x, y } => {
                self.register[x as usize] = self.register[y as usize];
                self.program_counter += 2;
            }
            Operation::BitwiseOr { x, y } => {
                self.register[x as usize] |= self.register[y as usize];
                self.program_counter += 2;
            }
            Operation::BitwiseAnd { x, y } => {
                self.register[x as usize] &= self.register[y as usize];
                self.program_counter += 2;
            }
            Operation::BitwiseXor { x, y } => {
                self.register[x as usize] ^= self.register[y as usize];
                self.program_counter += 2;
            }
            Operation::AddValues { x, y } => {
                let mut value: u16 = self.register[x as usize] as u16;
                value += self.register[y as usize] as u16;

                if value > 255 {
                    self.register[0xF] = 1;
                } else {
                    self.register[0xF] = 0;
                }

                self.register[x as usize] = value as u8;
                self.program_counter += 2;
            }
            Operation::SubtractValues { x, y } => {
                if self.register[x as usize] > self.register[y as usize] {
                    self.register[0xF] = 1;
                } else {
                    self.register[0xF] = 0;
                }

                self.register[x as usize] -= self.register[y as usize];
                self.program_counter += 2;
            }
            Operation::StoreLeastSignificant { x, .. } => {
                self.register[0xF] = (self.register[x as usize] & 0x1);
                self.register[x as usize] >>= 1;
            }
            Operation::SubtractValueFromRegister { x, y } => {
                if self.register[y as usize] > self.register[x as usize] {
                    self.register[0xF] = 1;
                } else {
                    self.register[0xF] = 0;
                }
                self.register[x as usize] = self.register[y as usize] - self.register[x as usize];
            }
            Operation::StoreMostSignificant { x, .. } => {
                self.register[0xF] = (self.register[x as usize] & 0x80) >> 7;
                self.register[x as usize] <<= 1;
            }
            Operation::InequalityRegisterCheck { x, y } => {
                if self.register[x as usize] != self.register[y as usize] {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            Operation::SetIndexToAddress { nnn } => {
                self.index = nnn;
                self.program_counter += 2;
            }
            Operation::GotoAddressWithRegister { nnn } => {
                self.program_counter = self.register[0] as u16 + nnn;
            }
            Operation::DrawSprite { x, y, n } => {
                let x_pos = self.register[x as usize] % 64;
                let y_pos = self.register[y as usize] % 32;

                self.register[0xF] = 0;

                for yline in 0..n {
                    let pixel = self.memory[(self.index + yline as u16) as usize];

                    for xline in 0..8 {
                        let sprite_pixel = pixel & (0x80 >> xline);
                        let screen_pixel = self.io.pixels[((y_pos as usize + yline as usize) * 64
                            + (x_pos as usize + xline as usize))
                            as usize];

                        if (sprite_pixel > 0) {
                            if screen_pixel > 0 {
                                self.register[0xF] = 1;
                            }
                            self.io.pixels[((y_pos as usize + yline as usize) * 64
                                + (x_pos as usize + xline as usize))
                                as usize] ^= 0xFF;
                        }
                    }
                }

                let mut data: Vec<u8> = vec![];

                for i in 0..(n as u16) {
                    let d = self.memory[(i + self.index) as usize];
                    data.push(d);
                }

                self.io.set_pixel(x.into(), y.into());
                self.program_counter += 2;
                self.draw_flag = true;
            }
            Operation::SetDelayTimer { x } => {
                println!("Setting timer: {}", x);
                self.program_counter += 2;
            }
            Operation::SetRegistersFromMemory { x } => {
                for i in 0..x {
                    self.register[i as usize] = self.memory[(self.index + i as u16) as usize];
                }
                self.program_counter += 2;
            }
            _ => {
                thread::sleep(Duration::from_secs(1));
                panic!("Unhandled: {:#0x}", opcode);
            }
        };
    }

    pub fn draw(&mut self) {
        self.io.clear();
        self.io.draw();
        self.io.present();
    }
}
