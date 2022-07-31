use std::fs;

use crate::Graphics;

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];


pub struct Emulator {
    graphics: Graphics,
    memory: [u8; 4096],
    registers: [u8; 16],
    i: u16,
    program_counter: u16,
    gfx: [u8; 64 * 32],
    stack: [u16; 16],
    stack_pointer: u16,
    delay_timer: u8,
    sound_timer: u8,
    key: [u8; 16],
    pub(crate) draw_flag: bool,
}

impl Emulator {
    pub fn new(graphics: Graphics) -> Emulator {
        let mut memory = [0; 4096];

        for (i, f) in FONT_SET.into_iter().enumerate() {
            memory[i] = f;
        }

        Emulator {
            graphics,
            program_counter: 0x200,
            memory,
            registers: [0; 16],
            i: 0,
            gfx: [0; 64 * 32],
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            key: [0; 16],
            draw_flag: false,
        }
    }

    pub fn load_game(&mut self, path: &str) {
        let rom = fs::read(path).expect("Unable to read from file");

        for (i, data) in rom.iter().enumerate() {
            self.memory[i + 0x200] = *data;
        }
    }

    pub fn emulate_cycle(&mut self) {
        let l = (self.memory[self.program_counter as usize] as u16) << 8;
        let r = self.memory[(self.program_counter + 1) as usize] as u16;
        let opcode: u16 = l | r;
        println!("{:#0x}", opcode);

        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x00FF {
                    0x0000 => self.program_counter += 2,
                    0x00E0 => {
                        self.graphics.clear();
                        self.program_counter += 2;
                    }
                    0x00EE => {
                        self.stack_pointer -= 1;
                        self.program_counter = self.memory[self.stack_pointer as usize] as u16;
                        self.program_counter += self.memory[self.stack_pointer as usize] as u16;
                    }
                    _ => panic!("Unhandled 0x00XX opcode: {:#0x}", opcode)
                }
            }
            0x1000 => {
                let jump = opcode & 0x0FFF;
                self.program_counter = jump;
            }
            0x2000 => {
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = opcode & 0x0FFF;
            }
            0x3000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let value = (opcode & 0x00FF >> 8) as u8;

                if self.registers[x] == value {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            0x4000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let value = (opcode & 0x00FF >> 8) as u8;
                if self.registers[x] != value {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            0x5000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.registers[x] == self.registers[y] {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            0x6000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let value = opcode & 0x00FF;
                self.registers[x] = value as u8;
                self.program_counter += 2;
            }
            0x7000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = ((opcode & 0x00FF) >> 8) as u8;
                self.registers[x] += nn;
                self.program_counter += 2;
            }
            0x9000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.registers[x] != self.registers[y] {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            0xA000 => {
                self.i = opcode & 0x0FFF;
                self.program_counter += 2;
            }
            0xD000 => {
                let x = self.registers[((opcode & 0x0F00) >> 8) as usize];
                let y = self.registers[((opcode & 0x00F0) >> 4) as usize];
                let height = (opcode & 0x000F) as u8;
                let mut pixel: u8;

                self.registers[0xf] = 0;

                for yline in 0..height {
                    pixel = self.memory[(self.i + yline as u16) as usize];
                    for xline in 0..8 {
                        let _i = (y + yline) as u16;
                        let index: usize = (x + xline + (_i * 64) as u8) as usize;
                        if self.gfx[index] == 2 {
                            self.registers[0xF] = 1;
                        }
                        self.gfx[index] ^= 1;
                    }
                }

                self.draw_flag = true;
                self.program_counter += 2;
            }
            0xF000 => {
                match opcode & 0xF0FF {
                    0xF029 => {
                        let loc = (opcode & 0x0FFF >> 8) as usize;
                        self.i = (self.registers[loc] * 5) as u16;
                        self.program_counter += 2;
                    }
                    0xF033 => {
                        let register_i = ((opcode & 0x0F00) >> 8) as usize;
                        self.memory[self.i as usize] = self.registers[register_i] / 100;
                        self.memory[(self.i + 1) as usize] = (self.registers[register_i] / 10) % 10;
                        self.memory[(self.i + 2) as usize] = (self.registers[register_i] % 100) % 10;
                        self.program_counter += 2;
                    }
                    0xF065 => {
                        let x = ((opcode & 0x0F00) >> 8) as usize;
                        for i in 0..=x {
                            self.registers[i] = self.memory[self.i as usize];
                            self.i += 1;
                        }
                        self.program_counter += 2;
                    }
                    _ => panic!("Unhandled 0xFXNN opcode: {:#0x}", opcode)
                }
            }
            _ => panic!("Unhandled opcode: {:#0x}", opcode)
        }
    }

    pub fn draw(&mut self) {
        self.graphics.clear();
        for (i, gfx) in self.gfx.iter().enumerate() {
            if *gfx == 1 {
                self.graphics.set_pixel((i % 32) as u8, (i / 32) as u8);
            }
        }
        println!("Draw");
        self.graphics.present();
    }

    pub fn set_keys(&self) {}
}
