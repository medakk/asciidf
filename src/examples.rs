use nalgebra_glm::{Vec2, Vec3};
extern crate nalgebra_glm as glm;

use crate::util::sdf;
use crate::renderer;
use crate::renderer::{Pixel};

pub fn simple_sdf(uniforms: &renderer::Uniforms, frag_coord: &Vec2) -> Pixel {
    let u = frag_coord.x / uniforms.resolution.x;
    let v = 1.0 - frag_coord.y / uniforms.resolution.y;

    let mut color;
    let mut ch;

    // Draw background
    {
        let a = Vec3::new(0.2, 0.7, 0.8);
        let b = Vec3::new(0.0, 0.9, 0.0);
        color = a*v + b*(1.0 - v);

        if (u*38.0) as isize % 2 == 0 { color *= 0.4; }
        if (v*18.0) as isize % 2 == 0 { color *= 0.8; }
        ch = '≣';
    }

    // Raymarch
    let shades = ['░', '▒', '▓'];
    {
        let dir = sdf::ray_dir(60.0, &uniforms.resolution, frag_coord);
        let eye = Vec3::new(0.0, 3.0, 10.5);

        let mut t = 0.0001;
        //TODO: Max dist
        for _ in 0..80 {
            let p = eye + t * &dir;
            let hit = sdf::map(&p);
            if hit.x < 1e-1  {
                let frames = uniforms.frames as f32 / 10.0;
                let light_pos = Vec3::new(40.0 * frames.cos(), 28.0 * frames.sin(), 20.0);
                color = sdf::shade(&p, hit.y, &light_pos, &-dir, sdf::map);

                let light_pos = Vec3::new(-40.0 * frames.cos(), 4.0 * frames.sin(), 8.0);
                color += sdf::shade(&p, hit.y, &light_pos, &-dir, sdf::map);

                let mag = glm::magnitude2(&glm::min(&color, 1.0));
                let mag_nor = mag / 1.7320508;
                let shades_idx = ((mag_nor.cbrt()* shades.len() as f32) as usize).min(shades.len()-1);
                ch = shades[shades_idx];
            } else {
                t += hit.x;
            }
        }
    }

    Pixel::new(ch, color)
}
