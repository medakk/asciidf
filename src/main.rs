mod renderer;
mod util;

use term_size;
use nalgebra_glm::{Vec2, Vec3, length2};

use renderer::{Pixel, Pixels};
use util::sdf;
use crate::util::sdf::ray_dir;

fn main() {
    let (w, h) = term_size::dimensions().unwrap_or((137, 28));
    let mut pixels = Pixels::new(w, h-1);

    pixels.update(shade);
    pixels.draw();
}

fn shade(uniforms: &renderer::Uniforms, frag_coord: &Vec2) -> Pixel {
    let u = frag_coord.x / uniforms.resolution.x;
    let v = 1.0 - frag_coord.y / uniforms.resolution.y;
    let chars = ['≣', '∞'];

    let mut pixel = Pixel::blank();

    // Draw background
    {
        let a = Vec3::new(0.2, 0.7, 0.8);
        let b = Vec3::new(0.0, 0.9, 0.0);
        pixel.color = a*v + b*(1.0 - v);
        if (u*38.0) as isize % 2 == 0 { pixel.color *= 0.4; }
        if (v*18.0) as isize % 2 == 0 { pixel.color *= 0.8; }
        pixel.ch = chars[0];
        // if length2(&(uv - Vec2::new(0.5, 0.5))) < 0.08 { pixel.ch = chars[1]; }
    }

    // Raymarch
    {
        let dir = ray_dir(60.0, &uniforms.resolution, frag_coord);
        let eye = Vec3::new(0.0, 3.0, 10.5);

        let mut t = 0.0001;
        //TODO: Max dist
        for i in 0..80 {
            let p = eye + t * &dir;
            let hit = sdf::map(&p);
            if hit.x < 1e-1  {
                pixel.color = Vec3::new(1.0, 1.0, 1.0);
                pixel.ch = chars[1];
            } else {
                t += hit.x;
            }
        }
    }

    pixel
}
