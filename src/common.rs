use num::{Float, FromPrimitive};
use rand::Rng;
use crate::vec3::*;

// IMAGE
pub const ASPECT_RATIO: f64 = 16. / 9.0;
pub const IMAGE_WIDTH: u32 = 400;
pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
pub const AA_SAMPLES: u32 = 100;

// Camera
pub const VIEWPORT_HEIGHT: f64 = 2.;
pub const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
pub const FOCAL_LENGTH: f64 = 1.;

// Threads
pub const NUM_THREADS: usize = 8;

pub fn random_in_unit_sphere<T>() -> Vec3<T>
where
    T: Float + FromPrimitive,
{
    let mut rng = rand::thread_rng();
    let mut p = Vec3::new(1., 1., 1.);
    while dot(p, p) >= 1. {
        p = Vec3::new(T::from_f64(rng.gen::<f64>()).unwrap(), T::from_f64(rng.gen::<f64>()).unwrap(), T::from_f64(rng.gen::<f64>()).unwrap()) * 2. - Vec3::new(1., 1., 1.);
    }
    p
}