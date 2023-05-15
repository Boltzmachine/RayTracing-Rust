use num::{Num, NumCast, FromPrimitive, Float};
use rand::Rng;
use std::ops::Neg;
use crate::vec3::*;

// Type
pub trait UVecElem: Num + FromPrimitive + NumCast + PartialOrd<Self> + Copy + Send + Sync {}
pub trait SVecElem: UVecElem + Neg<Output = Self> {}

impl UVecElem for f64 {}
impl UVecElem for f32 {}
impl UVecElem for u8 {}


impl SVecElem for f64 {}
impl SVecElem for f32 {}

// Image
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
    T: SVecElem + Float,
{
    let mut rng = rand::thread_rng();
    let mut p: Vec3<T> = Vec3::new(1., 1., 1.);
    while dot(&p, &p) >= T::from_f64(1.0).unwrap() {
        p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(),rng.gen::<f64>()) * T::from_f64(2.).unwrap() - Vec3::new(1., 1., 1.);
    }
    p.to_unit()
}