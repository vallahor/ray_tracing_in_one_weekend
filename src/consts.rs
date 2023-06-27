use rand::Rng;

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}

pub fn random() -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen::<f32>();
}

pub fn random_range(min: f32, max: f32) -> f32 {
    return min + (max - min) * random();
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}
