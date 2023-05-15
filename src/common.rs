use num::{Num, NumCast, FromPrimitive, Float, Signed, ToPrimitive};
use rand::Rng;
use std::ops::Neg;
use crate::vec3::*;

// Type
pub trait UVecElem: Num + ToPrimitive + FromPrimitive + NumCast + PartialOrd<Self> + Copy + Send + Sync {}
pub trait SVecElem: UVecElem + Neg<Output = Self> + Signed {}

impl UVecElem for f64 {}
impl UVecElem for f32 {}
impl UVecElem for u8 {}


impl SVecElem for f64 {}
impl SVecElem for f32 {}

// Image
pub const ASPECT_RATIO: f64 = 3. / 2.0;
pub const IMAGE_WIDTH: u32 = 1200;
pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
pub const AA_SAMPLES: u32 = 500;

// Threads
pub const NUM_THREADS: usize = 20;

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

pub fn random_in_unit_disk<T>() -> Vec3<T>
where
    T: SVecElem + Float,
{
    let mut rng = rand::thread_rng();

    loop {
        let p = Vec3::<T>::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.);
        if dot(&p, &p) < T::from_f64(1.0).unwrap() {
            return p;
        }
    }
}