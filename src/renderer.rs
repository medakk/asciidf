use wasm_bindgen::prelude::*;

use nalgebra_glm::{Vec3, Vec2, U8Vec3};
use colored::*;

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

pub struct Pixels {
    width: usize,
    height: usize,
    data: Vec<Pixel>,
}

pub struct Uniforms {
    pub resolution: Vec2
}

type Shader = fn (&Uniforms, &Vec2) -> Pixel;

impl Pixels {
    pub fn new(width: usize, height: usize) -> Pixels {
        let data = vec![Pixel::blank(); width*height];
        Pixels {
            width,
            height,
            data,
        }
    }

    pub fn update(&mut self, shader_func: Shader) {
        for y in 0..self.height {
            for x in 0..self.width {
                let uv = Vec2::new(
                    x as f32 + 0.5,
                    (self.height - y) as f32 + 0.5,
                );
                //TODO: How to account for each rendered char being taller than wider?
                let uniforms = Uniforms {
                    resolution: Vec2::new(self.width as f32, self.height as f32)
                };
                let idx = y*self.width + x;
                self.data[idx] = shader_func(&uniforms, &uv);
            }
        }
    }

    pub fn draw(&self) {
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
                print!("{}", s);
            }
            println!();
        }
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
}

