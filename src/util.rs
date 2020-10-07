// Most functions adapted from:
// https://www.iquilezles.org/www/index.htm
// http://jamie-wong.com/2016/07/15/ray-marching-signed-distance-functions/

// SDF Functions
#[allow(dead_code)]
#[allow(non_snake_case)]
pub mod sdf {
    extern crate nalgebra_glm as glm;
    use nalgebra_glm::{Vec2, Vec3};

    pub fn mod_f32(x: f32, y: f32) -> f32 {
        x - y * (x / y).floor()
    }

    pub fn clamp_f32(x: f32, a: f32, b: f32) -> f32 {
        x.min(b).max(a)
    }

    pub fn mix_f32(a: f32, b: f32, t: f32) -> f32 {
        t*a + (1.0 - t) * b
    }

    pub fn mix_vec2(a: &Vec2, b: &Vec2, t: f32) -> Vec2 {
        (1.0 - t)*a + t * b
    }

    pub fn mod_vec3(x: &Vec3, y: f32) -> Vec3 {
        x - glm::floor(&(x / y))
    }

    pub fn mod_vec3_vec3(x: &Vec3, y: &Vec3) -> Vec3 {
        x - y.component_mul(&glm::floor(&x.component_div(y)))
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

    pub fn smooth_sub(s1: &Vec2, s2: &Vec2, k: f32) -> Vec2 {
        let h = clamp_f32(0.5 - 0.5*(s2.x+s1.x)/k, 0.0, 1.0);
        let mut v = mix_vec2( s2,&vec2(-s1.x, s1.y), h );
        v.x += k*h*(1.0-h);
        v
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
            n += e*map_func(&(p+e*h)).x;
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
}
