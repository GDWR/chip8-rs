use sdl2::event::{Event, EventPollIterator};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::{EventPump, VideoSubsystem};
use std::unreachable;

const DISPLAY_WIDTH: u8 = 64;
const DISPLAY_HEIGHT: u8 = 32;

pub struct InputOutput {
    pub pixels: [u8; 32 * 64],
    scale: u32,
    video: VideoSubsystem,
    canvas: WindowCanvas,
    foreground: Color,
    background: Color,
    events: EventPump,
}

impl Default for InputOutput {
    fn default() -> Self {
        let scale = 8;
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "Chip8",
                scale * DISPLAY_WIDTH as u32,
                scale * DISPLAY_HEIGHT as u32,
            )
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        InputOutput {
            scale,
            pixels: [0; 32 * 64],
            video: video_subsystem,
            canvas,
            events: sdl_context.event_pump().unwrap(),
            foreground: Color::RGB(150, 150, 35),
            background: Color::RGB(20, 20, 0),
        }
    }
}

impl InputOutput {
    pub fn present(&mut self) {
        self.canvas.present()
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.background);
        self.canvas.clear()
    }

    pub fn draw(&mut self) {
        self.canvas.set_draw_color(self.foreground);

        for (i, pixel) in self.pixels.iter().enumerate() {
            if *pixel > 0 {
                let x = (i % 64) as u32;
                let y = ((i / 64) % 64) as u32;

                let rect = Rect::from((
                    (self.scale * x) as i32,
                    (self.scale * y) as i32,
                    self.scale,
                    self.scale,
                ));

                self.canvas.fill_rect(rect).expect("TODO: panic message");
            }
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        self.pixels[((y * 64) + x) as usize] = 1;
    }

    pub fn iter_events(&mut self) -> EventPollIterator {
        self.events.poll_iter()
    }

    pub fn await_keypress(&mut self) -> Keycode {
        for event in self.events.wait_iter() {
            if let Event::KeyDown { keycode, .. } = event {
                return keycode.unwrap();
            }
        }
        unreachable!();
    }
}
