mod common;
mod vec3;
mod camera;
mod ray;
mod hittable;
mod hittable_list;

use common::*;
use vec3::*;
use camera::*;
use ray::*;
use hittable_list::*;
use num::{Float, FromPrimitive};
use hittable::*;
use std::convert::Into;
use std::f64::INFINITY;


fn float_to_rgb_vec<T>(r: T, g: T, b: T) -> Color3<u8>
where
    T: Float + FromPrimitive
{
    let r = T::from_f64(255.999).unwrap() * r;
    let g = T::from_f64(255.999).unwrap() * g;
    let b = T::from_f64(255.999).unwrap() * b;

    Color3(r.to_u8().unwrap(), g.to_u8().unwrap(), b.to_u8().unwrap())
}

fn write_color<T>(pixel_color: Color3<T>)
where
    T: Copy + Float + FromPrimitive,
{
    let Color3(r, g, b) = float_to_rgb_vec(pixel_color.x(), pixel_color.y(), pixel_color.z());
    println!("{} {} {}", r, g, b);
}


fn ray_color<T>(ray: Ray<T>, world: &HittableList<T>) -> Color3<T>
where
    T: Float + FromPrimitive,
{   
    let t = world.hit(&ray, T::from_f64(0.0).unwrap(), T::from_f64(INFINITY).unwrap());
    match t {
        Some(hit) => {
            let normal = (ray.at(hit.t) - Vec3::new(0.0, 0.0, -1.0)).to_unit();
            (Color3::new(1.0, 1.0, 1.0) + normal) * T::from_f64(0.5).unwrap()
        },
        None => {
            let unit_direction = ray.direction.to_unit();
            let t = (unit_direction.y() + T::from_f64(1.0).unwrap()) * T::from_f64(0.5).unwrap();
            Color3::new(1.0, 1.0, 1.0) * (T::from_f64(1.0).unwrap() - t) + Color3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn render_single<T> (image: &[Color3<T>], world: &HittableList<T>, cam: &Camera<T>)
where
    T: Float + FromPrimitive,
{
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            
            let ray = cam.get_ray(u, v);

            let pixel_color = ray_color(ray, &world);
            image[(j * IMAGE_WIDTH + i) as usize] += pixel_color;
        }
    }
}

fn write_image<T>(image: &[Color3<T>]) 
where 
    T: Float + FromPrimitive,
{
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let pixel_color = image[(j * IMAGE_WIDTH + i) as usize];
            write_color(pixel_color);
        }
    }
}

fn main() {
    // World
    let world: HittableList::<f64> = vec![
        Box::new(Sphere {
                    center: Point3::new(0.0, 0.0, -1.0),
                    radius: 0.5,
                }),
        Box::new(Sphere {
                    center: Point3::new(0.0, -100.5, -1.0),
                    radius: 100.0,
                }),
        ];

    let cam = Camera::<f64>::default();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let image = vec![Color3::<f64>::new(0., 0., 0.); (IMAGE_HEIGHT * IMAGE_WIDTH) as usize];

    render_single(&image, &world, &cam);

}
