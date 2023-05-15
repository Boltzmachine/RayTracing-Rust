mod common;
mod vec3;
mod camera;
mod ray;
mod hittable;
mod hittable_list;
mod image;
mod materials;

use common::*;
use num::Float;
use vec3::*;
use camera::*;
use ray::*;
use hittable_list::*;
use hittable::*;
use image::*;
use materials::*;

use std::f64::INFINITY;
use threadpool::ThreadPool;
use std::sync::{mpsc, Arc, Mutex};
use rand::Rng;


fn ray_color<T>(ray: Ray<T>, world: &HittableList<T>, depth: u32) -> Color3<T>
where
    T: SVecElem + Float,
{   
    if depth == 0 {
        return Color3::new(0.0, 0.0, 0.0);
    }
    let t = world.hit(&ray, T::from_f64(0.001).unwrap(), T::from_f64(INFINITY).unwrap());
    match t {
        Some(hit) => {
            let hit_result = hit.material.scatter(&ray, &hit);
            match hit_result {
                Some((attenuation, scattered)) => {
                    attenuation * ray_color(scattered, world, depth - 1)
                },
            None => Color3::new(0.0, 0.0, 0.0)
            }
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
    T: SVecElem + Float,
{   
    let length = (IMAGE_WIDTH * IMAGE_HEIGHT) as usize;
    let mut image = Vec::with_capacity(length);
    const MAX_BOUNCE: u32 = 50;

    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64 + rng.gen_range(-0.5..0.5)) / (IMAGE_WIDTH - 1) as f64;
            let v = (j as f64 + rng.gen_range(-0.5..0.5)) / (IMAGE_HEIGHT - 1) as f64;
            
            let ray = cam.get_ray(u, v);

            let pixel_color = ray_color(ray, &world, MAX_BOUNCE);
            image.push(pixel_color);
        }
    }
    Box::new(image)
}


macro_rules! create_material {
    ("lambertian", ($r:expr, $g:expr, $b:expr)) => {
        Arc::new(Lambertian::<f64> {albedo: Color3::new($r, $g, $b)})
    };
    ("metal", ($r:expr, $g:expr, $b:expr), $f:expr) => {
        Arc::new(Metal::<f64> {albedo: Color3::new($r, $g, $b), fuzz: $f})
    };
    ("dielectric", $ir:expr) => {
        Arc::new(Dielectric::<f64> {ir: $ir})
    }
}

macro_rules! create_objects {
    ("sphere", $center:expr, $radius:expr, $material:expr) => {
        Box::new(Sphere {
                    center: $center,
                    radius: $radius,
                    material: Arc::clone(&$material) as _,
                })
    };
}

fn main() {

    // Materials
    let material_ground = create_material!("lambertian", (0.8, 0.8, 0.0));
    let material_center = create_material!("dielectric", 1.5);
    let material_left = create_material!("dielectric", 1.5);
    let material_right = create_material!("metal", (0.8, 0.6, 0.2), 1.0);

    // World
    let world: HittableList::<'_, f64> = vec![
        create_objects!("sphere", Point3::new(0.0, 0.0, -1.0), 0.5, material_center),
        create_objects!("sphere", Point3::new(0.0, -100.5, -1.0), 100.0, material_ground),
        create_objects!("sphere", Point3::new(-1.0, 0.0, -1.0), 0.5, material_left),
        create_objects!("sphere", Point3::new(1.0, 0.0, -1.0), 0.5, material_right),
        ];

    // Camera
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
                eprintln!("{} jobs left", *thread_job_left);
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
            image[i] += single[i];
        }
    }

    eprintln!("All images reduced, writing image...");
    write_image(&image);
}
