// Some useful references:
// https://www.iquilezles.org/www/index.htm
// http://jamie-wong.com/2016/07/15/ray-marching-signed-distance-functions/

// SDF Functions
#[allow(dead_code)]
pub mod sdf {
    use nalgebra_glm::{Vec2, Vec3, length, max, abs, min, normalize};

    pub fn vec2(x: f32, y: f32) -> Vec2 {
        Vec2::new(x, y)
    }

    pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3::new(x, y, z)
    }

    pub fn sphere(p: &Vec3, s: f32) -> f32 {
        length(p) - s
    }

    pub fn plane(p: &Vec3) -> f32 {
        p.y
    }

    // `box` is reserved :p
    pub fn boxy(p: &Vec3, b: &Vec3) -> f32
    {
        let q = abs(p) - b;
        length(&max(&q,0.0)) + (q.x.max(q.y.max(q.z))).min(0.0)
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

        normalize(&vec3(xy.x, xy.y, -z))
    }

    //TODO: abstract out
    pub fn map(p: &Vec3) -> Vec2 {
        let mut s = vec2(1e10, 0.0);

        s = union(&s, &vec2(sphere(&(p - vec3(0.0, 2.3, 0.0)), 4.3), 1.0));
        s = union(&s, &vec2(boxy(&(p - vec3(18.0, 2.3, 0.0)), &vec3(1.0, 1.0, 1.0)), 1.0));

        s
    }
}
