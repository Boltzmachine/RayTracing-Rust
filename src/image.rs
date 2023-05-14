use num::{Float, FromPrimitive};

use crate::vec3::*;

pub type Image<T> = Box<Vec<Color3<T>>>;

pub fn float_to_rgb_vec<T>(r: T, g: T, b: T) -> Color3<u8>
where
    T: Float + FromPrimitive
{
    let r = T::from_f64(255.999).unwrap() * r;
    let g = T::from_f64(255.999).unwrap() * g;
    let b = T::from_f64(255.999).unwrap() * b;

    Color3(r.to_u8().unwrap(), g.to_u8().unwrap(), b.to_u8().unwrap())
}

pub fn write_color<T>(pixel_color: Color3<T>)
where
    T: Copy + Float + FromPrimitive,
{
    let Color3(r, g, b) = float_to_rgb_vec(pixel_color.x(), pixel_color.y(), pixel_color.z());
    println!("{} {} {}", r, g, b);
}


pub fn write_image<T>(image: &[Color3<T>]) 
where 
    T: Float + FromPrimitive,
{
    for pixel_color in image.iter() {
        write_color(*pixel_color);
    }
}