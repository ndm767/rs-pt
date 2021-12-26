use crate::linalg::Vec3;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Display {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Vec3>>,
    pub pixel_iters: Vec<Vec<i64>>,
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,
    keys_down: Vec<Keycode>,
    pub should_quit: bool,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Display {
        // set up pixel array
        let mut pixels: Vec<Vec<Vec3>> = vec![];
        let mut pixel_iters: Vec<Vec<i64>> = vec![];
        for _x in 0..width {
            let mut row: Vec<Vec3> = vec![];
            let mut row_iters: Vec<i64> = vec![];
            for _y in 0..height {
                row.push(Vec3::new(0.0, 0.0, 0.0));
                row_iters.push(0);
            }
            pixels.push(row);
            pixel_iters.push(row_iters);
        }
        // set up sdl canvas
        let context = sdl2::init().unwrap();
        let sdl_video = context.video().unwrap();
        let window = sdl_video
            .window("rs-pt", width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();
        Display {
            width,
            height,
            pixels,
            pixel_iters,
            canvas: window.into_canvas().build().unwrap(),
            event_pump: context.event_pump().unwrap(),
            keys_down: vec![],
            should_quit: false,
        }
    }

    // draw pixels and display
    pub fn show(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        // draw each pixel
        for x in 0..self.width {
            for y in 0..self.height {
                // get pixel color
                let mut color: (f64, f64, f64) = self.pixels[x][y].to_tuple();
                // divide color by number of iterations (samples) per pixel
                let iters = self.pixel_iters[x][y];
                if iters != 0 {
                    color.0 /= iters as f64;
                    color.1 /= iters as f64;
                    color.2 /= iters as f64;
                }

                // convert color from 0.0-1.0 f64 internal format to 0-255 u8 sdl2 format
                let converted_color: (u8, u8, u8) = (
                    (color.0 * 255.0) as u8,
                    (color.1 * 255.0) as u8,
                    (color.2 * 255.0) as u8,
                );

                // draw pixel
                self.canvas.set_draw_color(Color::from(converted_color));
                self.canvas
                    .draw_point(Point::from((x as i32, y as i32)))
                    .expect("could not draw point!\n");
            }
        }
        // show canvas
        self.canvas.present();
    }

    pub fn poll_events(&mut self) {
        self.keys_down.clear();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.should_quit = true,
                Event::KeyDown {
                    keycode: Some(k), ..
                } => self.keys_down.push(k),
                _ => {}
            }
        }
    }

    // not currently used but I might want this in the future
    pub fn _is_key_down(&mut self, keycode: Keycode) -> bool {
        self.keys_down.contains(&keycode)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        self.pixels[x][y] += color;
        self.pixel_iters[x][y] += 1;
    }
}
