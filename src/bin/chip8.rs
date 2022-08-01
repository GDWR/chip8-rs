use chip8::system::System;
use clap::Parser;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    rom: String,

    /// Steps per second
    #[clap(short, long, value_parser, default_value_t = 30)]
    sps: u16,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let path = Path::new(args.rom.as_str());
    println!("Loading {}", path.display());

    let mut system = System::default();

    system.load_rom_from_file(path);

    'main: loop {
        for event in system.iter_events() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        system.step();

        if system.draw_flag {
            system.draw();
            system.draw_flag = false;
        }

        sleep(Duration::from_millis((1000 / args.sps).into()));
    }

    Ok(())
}
