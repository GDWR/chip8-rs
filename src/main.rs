use std::fs;
use std::thread::sleep;
use std::time::Duration;
use emulator::Emulator;

use crate::graphics::Graphics;

mod graphics;
mod opcode;
mod emulator;


fn main() {
    let graphics = Graphics::new();

    let mut emulator = Emulator::new(graphics);

    emulator.load_game("/home/greg/Repos/chip8/games/test_opcode.ch8");

    loop {
        sleep(Duration::from_millis(50));
        emulator.emulate_cycle();

        if emulator.draw_flag {
            emulator.draw_flag = false;
            emulator.draw();
        }

        emulator.set_keys();
    }
}
