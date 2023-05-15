use num::{Float, clamp};

use crate::vec3::*;
use crate::common::*;

pub type Image<T> = Box<Vec<Color3<T>>>;

pub fn float_to_rgb_vec<T>(r: T, g: T, b: T) -> Color3<u8>
where
    T: SVecElem + Float,
{
    let scale = T::from_f64(1.0 / AA_SAMPLES as f64).unwrap();
    let clamp_min = T::from_f64(0.0).unwrap();
    let clamp_max = T::from_f64(0.9999).unwrap();

    let r = clamp((scale * r).sqrt(), clamp_min, clamp_max) * T::from_i32(256).unwrap();
    let g = clamp((scale * g).sqrt(), clamp_min, clamp_max) * T::from_i32(256).unwrap();
    let b = clamp((scale * b).sqrt(), clamp_min, clamp_max) * T::from_i32(256).unwrap();

    Color3(r.to_u8().unwrap(), g.to_u8().unwrap(), b.to_u8().unwrap())
}

pub fn write_color<T>(pixel_color: Color3<T>)
where
    T: SVecElem + Float,
{
    let Color3(r, g, b) = float_to_rgb_vec(pixel_color.x(), pixel_color.y(), pixel_color.z());
    println!("{} {} {}", r, g, b);
}


pub fn write_image<T>(image: &[Color3<T>]) 
where 
    T: SVecElem + Float,
{
    for pixel_color in image.iter() {
        write_color(*pixel_color);
    }
}