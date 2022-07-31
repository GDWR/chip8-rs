use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

pub struct Graphics {
    canvas: WindowCanvas,
    scale: u8,
}

impl Graphics {
    pub fn new() -> Graphics {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let scale: u8 = 20;
        let window = video_subsystem.window("Chip8", 64 * (scale as u32), 32 * (scale as u32))
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window
            .into_canvas()
            .build()
            .unwrap();

        return Graphics { canvas, scale };
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        println!("Present");
        self.canvas.present()
    }

    pub fn set_pixel(&mut self, x: u8, y: u8) {
        self.canvas.set_draw_color(Color::RGB(255 ,255, 0));
        self.canvas
            .fill_rect(Rect::new((x as i32 * self.scale as i32), (y as i32 * self.scale as i32), self.scale as u32, self.scale as u32))
            .expect("Can't draw");
    }
}

