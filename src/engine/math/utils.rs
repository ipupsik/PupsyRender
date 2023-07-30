use glam::{Vec3A};
use rand::{Rng};

fn random(min : f32, max : f32) -> Vec3A {
    Vec3A::new(rand::thread_rng().gen_range(min..max), 
        rand::thread_rng().gen_range(min..max), 
        rand::thread_rng().gen_range(min..max))
}

pub fn random_in_unit_sphere() -> Vec3A {
    loop {
        let point = random(-1.0, 1.0);
        if point.length_squared() <= 1.0 {
            return point;
        }
    }
}