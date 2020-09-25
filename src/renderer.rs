use nalgebra_glm::{Vec3, Vec2, U8Vec3, length2};
use colored::*;

#[derive(Clone, Copy)]
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

impl Pixels {
    pub fn new(width: usize, height: usize) -> Pixels {
        let data = vec![Pixel::blank(); width*height];
        Pixels {
            width,
            height,
            data,
        }
    }

    pub fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let uv = Vec2::new(
                    (x as f32 + 0.5) / self.width as f32,
                    (y as f32 + 0.5) / self.height as f32,
                );
                let idx = y*self.width + x;
                self.data[idx] = shade(uv);
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
}

//TODO: caller provides this function
fn shade(uv: Vec2) -> Pixel {
    let u = uv[0];
    let v = 1.0 - uv[1];

    let chars = ['≣', '∞'];

    let a = Vec3::new(0.2, 0.7, 0.8);
    let b = Vec3::new(0.0, 0.9, 0.0);
    let mut color = a*v + b*(1.0 - v);

    if (u*38.0) as isize % 2 == 0 {
        color *= 0.4;
    }
    if (v*18.0) as isize % 2 == 0 {
        color *= 0.8;
    }

    let mut ch = chars[0];
    if length2(&(uv - Vec2::new(0.5, 0.5))) < 0.08 {
        ch = chars[1];
    }

    return Pixel::new(ch, color);
}
