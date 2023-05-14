mod common;
mod vec3;
mod camera;
mod ray;
mod hittable;
mod hittable_list;
mod image;

use common::*;
use vec3::*;
use camera::*;
use ray::*;
use hittable_list::*;
use num::{Float, FromPrimitive};
use hittable::*;
use image::*;

use std::f64::INFINITY;
use threadpool::ThreadPool;
use std::sync::{mpsc, Arc, Mutex};
use rand::Rng;


fn ray_color<T>(ray: Ray<T>, world: &HittableList<T>) -> Color3<T>
where
    T: Float + FromPrimitive,
{   
    let t = world.hit(&ray, T::from_f64(0.0).unwrap(), T::from_f64(INFINITY).unwrap());
    match t {
        Some(hit) => {
            let target = hit.p + hit.normal + random_in_unit_sphere();
        },
        None => {
            let unit_direction = ray.direction.to_unit();
            let t = (unit_direction.y() + T::from_f64(1.0).unwrap()) * T::from_f64(0.5).unwrap();
            Color3::new(1.0, 1.0, 1.0) * (T::from_f64(1.0).unwrap() - t) + Color3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn render_single<T> (world: &HittableList<T>, cam: &Camera<T>) -> Image<T>
where
    T: Float + FromPrimitive,
{   
    let length = (IMAGE_WIDTH * IMAGE_HEIGHT) as usize;
    let mut image = Vec::with_capacity(length);

    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64 + rng.gen_range(-0.5..0.5)) / (IMAGE_WIDTH - 1) as f64;
            let v = (j as f64 + rng.gen_range(-0.5..0.5)) / (IMAGE_HEIGHT - 1) as f64;
            
            let ray = cam.get_ray(u, v);

            let pixel_color = ray_color(ray, &world);
            image.push(pixel_color);
        }
    }
    Box::new(image)
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

    let pool = ThreadPool::new(NUM_THREADS);
    let (tx, rx) = mpsc::channel::<Image<f64>>();

    let job_left = Arc::new(Mutex::new(AA_SAMPLES));

    let world = Arc::new(world);
    let cam = Arc::new(cam);
    for _ in 0..AA_SAMPLES {
        let thread_tx = tx.clone();
        let thread_world = Arc::clone(&world);
        let thread_cam = Arc::clone(&cam);
        let thread_job_left = Arc::clone(&job_left);

        pool.execute(move || {
                let image = render_single(&thread_world, &thread_cam);
                let mut thread_job_left = thread_job_left.lock().unwrap();
                *thread_job_left -= 1;
                // eprintln!("{} jobs left", *thread_job_left);
                drop(thread_job_left);
                thread_tx.send(image).unwrap();
            }
        );
    }

    let mut image = vec![Color3::<f64>::new(0.0, 0.0, 0.0); (IMAGE_WIDTH * IMAGE_HEIGHT) as usize];

    for _ in 0..AA_SAMPLES {
        let single = rx.recv().unwrap();
        assert_eq!(single.len(), image.len());
        for i in 0..single.len() {
            image[i] += single[i] / AA_SAMPLES as f64;
        }
    }

    eprintln!("All images reduced, writing image...");
    write_image(&image);
}
