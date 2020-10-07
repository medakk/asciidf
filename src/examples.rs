use nalgebra_glm::{Vec2, Vec3};
extern crate nalgebra_glm as glm;

use crate::util::sdf::*;
use crate::renderer;
use crate::renderer::{Pixel};

pub fn box_minux_sphere(uniforms: &renderer::Uniforms, frag_coord: &Vec2) -> Pixel {
    let u = frag_coord.x / uniforms.resolution.x;
    let v = 1.0 - frag_coord.y / uniforms.resolution.y;

    let mut color;
    let mut ch;

    // Draw background
    {
        color = Vec3::new(0.2, 0.2, 0.2);
        ch = '≣';
    }

    // Raymarch
    let shades = ['░', '▒', '▓'];
    {
        let dir = ray_dir(60.0, &uniforms.resolution, frag_coord);
        let eye = Vec3::new(0.0, 3.0, 10.5 - uniforms.frames as f32*0.8);

        let mut t = 0.0001;
        //TODO: Max dist
        for _ in 0..300 {
            let p = eye + t * &dir;
            let hit = box_minus_sphere_map(&p);
            if hit.x < 1e-5  {
                let frames = uniforms.frames as f32 / 10.0;
                let light_pos = p + Vec3::new(40.0, 28.0, 20.0);
                color = shade(&p, hit.y, &light_pos, &-dir, box_minus_sphere_map);

                let light_pos = p + Vec3::new(1.0, 3.0, -2.0);
                color += shade(&p, hit.y, &light_pos, &-dir, box_minus_sphere_map);

                let light_pos = p + Vec3::new(0.0, 0.0, -8.0);
                color += shade(&p, hit.y, &light_pos, &-dir, box_minus_sphere_map);

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

fn box_minus_sphere_map(p: &Vec3) -> Vec2 {
    let mut s = vec2(1e10, 0.0);

    // for repetition
    {
        let c = vec3(20.2, 10000.0, 30.0);
        let p2 = mod_vec3_vec3(&(p + 0.5 * &c), &c) - 0.5 * &c;

        s = union(&s, &vec2(sphere(&(&p2 - &vec3(0.0, 3.0, 0.0)), 3.3), 1.0));
        s = smooth_sub(&s, &vec2(boxy(&(&p2 - &vec3(0.0, 3.0, 0.0)),&vec3(3.0, 3.0, 3.0)), 1.0), 0.5);
    }

    s = union(&s, &vec2(plane(&p), 0.2));
    s
}
