#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use wasm_bindgen::prelude::*;
use std::io::{stdout, Write};

use nalgebra_glm::{Vec3, Vec2, U8Vec3};
use colored::*;
use ncurses;

//TODO REMOVE
use crate::examples::simple_sdf;

#[wasm_bindgen]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Pixel {
    ch: char,
    color: Vec3,
}

impl Pixel {
    pub fn new(ch: char, color: Vec3) -> Pixel {
        Pixel {
            ch,
            color,
        }
    }

    pub fn blank() -> Pixel {
        Pixel {
            ch: ' ',
            color: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

#[wasm_bindgen]
pub struct Pixels {
    width: usize,
    height: usize,
    data: Vec<Pixel>,
    frames: usize,
}

pub struct Uniforms {
    pub resolution: Vec2,
    pub frames: usize,
}

type Shader = fn (&Uniforms, &Vec2) -> Pixel;

impl Pixels {


    pub fn update(&mut self, shader_func: Shader) {
        //TODO Can we do into_par_iter without allocating a new array?

        #[cfg(not(target_arch = "wasm32"))]
        let iter = (0..self.width*self.height).into_par_iter();
        #[cfg(target_arch = "wasm32")]
        let iter = (0..self.width*self.height).into_iter();

        self.data = iter.map(|idx| {
            let x = idx % self.width;
            let y = idx / self.width;
            let uv = Vec2::new(
                x as f32 + 0.5,
                (self.height - y) as f32 + 0.5,
            );
            //TODO: How to account for each rendered char being taller than wider?
            let uniforms = Uniforms {
                resolution: Vec2::new(self.width as f32, self.height as f32),
                frames: self.frames,
            };
            shader_func(&uniforms, &uv)
        }).collect();

        self.frames += 1;
    }

    pub fn draw(&self) {
        let mut stdout = stdout();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y*self.width + x;
                let pixel = &self.data[idx];
                let c_u8 = U8Vec3::new(
                    (pixel.color[0] * 255.0) as u8, // rust will auto clamp
                    (pixel.color[1] * 255.0) as u8,
                    (pixel.color[2] * 255.0) as u8,
                );

                //TODO: Is this the best way?
                let s = pixel.ch.to_string().truecolor(c_u8[0], c_u8[1], c_u8[2]);
                stdout.write_all(s.to_string().as_bytes()).unwrap();
            }
            stdout.write_all("\n".as_bytes()).unwrap();
        }
        stdout.flush().unwrap();
    }

    pub fn ncurses_draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y*self.width + x;
                let pixel = &self.data[idx];
                let c_u8 = U8Vec3::new(
                    (pixel.color[0] * 255.0) as u8, // rust will auto clamp
                    (pixel.color[1] * 255.0) as u8,
                    (pixel.color[2] * 255.0) as u8,
                );

                ncurses::mvprintw(y as i32, x as i32, &pixel.ch.to_string());
            }
        }
        ncurses::refresh();
    }

}

#[wasm_bindgen]
impl Pixels {
    pub fn new(width: usize, height: usize) -> Pixels {
        let data = vec![Pixel::blank(); width*height];
        Pixels {
            width,
            height,
            data,
            frames: 0,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width == width && self.height == height {
            return;
        }

        self.width = width;
        self.height = height;
        self.data = vec![Pixel::blank(); width*height];
    }
    pub fn html(&self) -> String {
        let mut s = String::with_capacity((self.width+1) * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let pixel = &self.data[idx];
                let c_u8 = U8Vec3::new(
                    (pixel.color[0] * 255.0) as u8, // rust will auto clamp
                    (pixel.color[1] * 255.0) as u8,
                    (pixel.color[2] * 255.0) as u8,
                );
                s.push_str(format!("<span style=\"color: rgb({}, {}, {})\">", c_u8[0], c_u8[1], c_u8[2]).as_str());
                s.push(pixel.ch);
                s.push_str("</span>");
            }
            s.push('\n');
        }
        s
    }

    pub fn hacky_update(&mut self) {
        self.update(simple_sdf);
    }

    pub fn write_to_buffer(&self, fetcher: &mut Fetcher)  {
        fetcher.buffer.resize(self.width * self.height * 2, 0);

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let pixel = &self.data[idx];
                let c_u8 = U8Vec3::new(
                    (pixel.color[0] * 255.0) as u8, // rust will auto clamp
                    (pixel.color[1] * 255.0) as u8,
                    (pixel.color[2] * 255.0) as u8,
                );

                fetcher.buffer[idx * 2] = pixel.ch as u32;
                fetcher.buffer[idx * 2 + 1] = 0xff << 24 + c_u8[0] << 16 + c_u8[1] << 8 + c_u8[2];
            }
        }
    }
}

#[wasm_bindgen]
pub struct Fetcher {
    buffer: Vec<u32>
}

#[wasm_bindgen]
impl Fetcher {
    pub fn new() -> Fetcher {
        Fetcher {
            buffer: Vec::new()
        }
    }
    pub fn fetch(&mut self) -> *const u32 {
        self.buffer.as_ptr()
    }
}
