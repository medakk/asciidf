// Super useful references:
// https://www.iquilezles.org/www/index.htm
// http://jamie-wong.com/2016/07/15/ray-marching-signed-distance-functions/

// SDF Functions
#[allow(dead_code)]
pub mod sdf {
    extern crate nalgebra_glm as glm;
    use nalgebra_glm::{Vec2, Vec3};

    pub fn mix_f32(a: f32, b: f32, t: f32) -> f32 {
        t*a + (1.0 - t) * b
    }

    pub fn vec2(x: f32, y: f32) -> Vec2 {
        glm::Vec2::new(x, y)
    }

    pub fn vec3(x: f32, y: f32, z: f32) -> glm::Vec3 {
        Vec3::new(x, y, z)
    }

    pub fn sphere(p: &Vec3, s: f32) -> f32 {
        glm::length(p) - s
    }

    pub fn plane(p: &Vec3) -> f32 {
        p.y
    }

    // `box` is reserved :p
    pub fn boxy(p: &Vec3, b: &Vec3) -> f32
    {
        let q = glm::abs(p) - b;
        glm::length(&glm::max(&q,0.0)) + (q.x.max(q.y.max(q.z))).min(0.0)
    }

    pub fn union(s1: &Vec2, s2: &Vec2) -> Vec2 {
        if s1.x < s2.x {
            s1.clone()
        } else {
            s2.clone()
        }
    }

    // get ray direction
    pub fn ray_dir(fov: f32, size: &Vec2, pos: &Vec2) -> Vec3 {
        let xy = pos - size * 0.5;

        let cot_half_fov = ((90.0 - fov * 0.5) * 57.2957795131).tan();
        let z = &size.y * 0.5 * cot_half_fov;

        glm::normalize(&vec3(xy.x, xy.y, -z))
    }

    type MapFunc = fn(&Vec3) -> Vec2;

    pub fn calc_normal(p: &Vec3, map_func: MapFunc) -> Vec3 {
        let h  = 0.0001;
        let mut n = vec3(0.0, 0.0, 0.0);
        for i in 0..4 {
            let e = 0.5773*(2.0*&vec3((((i+3)>>1)&1) as f32,((i>>1)&1) as f32,(i&1) as f32) - &vec3(1.0, 1.0, 1.0));
            n += e*map(&(p+e*h)).x;
        }
        glm::normalize(&n)
    }

    pub fn shade(p: &Vec3, mat: f32, light_pos: &Vec3, view: &Vec3, map_func: MapFunc) -> Vec3 {
        let N = calc_normal(p, map_func);
        let L= glm::normalize(&(light_pos - p));
        let V = view;
        let R = glm::normalize(&(2.0 * glm::dot(&L, &N)*N - L));

        //TODO: Ambient light
        let id = (0.2*glm::sin(&vec3(0.0,1.0,2.0).add_scalar(mat * 4.0))).add_scalar(0.7);
        let is = (0.2*glm::sin(&vec3(0.0,1.0,2.0).add_scalar(mat * 4.0))).add_scalar(0.7);
        let alpha = mix_f32(0.3, 7.0, mat);
        let kd = mix_f32(0.8, 0.5, mat);
        let ks = mix_f32(0.2, 1.2, mat);

        let dotLN = glm::dot(&L, &N).max(0.0);
        let diff = dotLN * id;
        let spec = dotLN.ceil() * (glm::dot(&R, &V).max(0.0)).powf(alpha) * is;

        kd*diff + ks*spec
    }

    //TODO: abstract out
    pub fn map(p: &Vec3) -> Vec2 {
        let mut s = vec2(1e10, 0.0);

        s = union(&s, &vec2(sphere(&(p - vec3(0.0, 5.3, 0.0)), 4.3), 1.0));
        s = union(&s, &vec2(boxy(&(p - vec3(18.0, 2.3, 0.0)), &vec3(1.0, 1.0, 1.0)), 1.0));
        s = union(&s, &vec2(plane(&p), 0.2));

        s
    }
}
