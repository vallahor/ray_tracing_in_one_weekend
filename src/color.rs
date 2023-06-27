use crate::consts::*;
use crate::math::vec3::Vec3;

pub fn write_color(pixel_color: &mut Vec3, samples_per_pixel: i32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f32;

    r = f32::sqrt(scale * r);
    g = f32::sqrt(scale * g);
    b = f32::sqrt(scale * b);

    pixel_color.x = 256.0 * clamp(r, 0.0, 0.999);
    pixel_color.y = 256.0 * clamp(g, 0.0, 0.999);
    pixel_color.z = 256.0 * clamp(b, 0.0, 0.999);
}
