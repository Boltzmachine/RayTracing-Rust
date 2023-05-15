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
    ("lambertian", $generic:ty, ($r:expr, $g:expr, $b:expr)) => {
        Arc::new(Lambertian::<$generic> {albedo: Color3::new($r, $g, $b)})
    };
    ("metal", $generic:ty, ($r:expr, $g:expr, $b:expr), $f:expr) => {
        Arc::new(Metal::<$generic> {albedo: Color3::new($r, $g, $b), fuzz: <$generic>::from_f64($f).unwrap()})
    };
    ("dielectric", $generic:ty, $ir:expr) => {
        Arc::new(Dielectric::<$generic> {ir: <$generic>::from_f64($ir).unwrap()})
    }
}

macro_rules! create_object {
    ("sphere", $generic:ty, $center:expr, $radius:expr, $material:expr) => {
        Box::new(Sphere::<$generic> {
                    center: Point3::<$generic>::new($center.0, $center.1, $center.2),
                    radius: <$generic>::from_f64($radius).unwrap(),
                    material: Arc::clone(&$material) as _,
                })
    };
}

fn random_scene<'a, T>() -> HittableList<'a, T>
where
    T: 'a + SVecElem + Float,
{
    let mut rng = rand::thread_rng();
    
    let mut world = HittableList::<T>::new();

    let ground_material = create_material!("lambertian", T, (0.5, 0.5, 0.5));
    world.push(create_object!("sphere", T, (0., -1000., 0.), 1000., ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let (center_x, center_y, center_z) = (a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());
            let center = Point3::<T>::new(center_x, center_y, center_z);

            if (center - Point3::<T>::new(4., 0.2, 0.)).length() > T::from_f64(0.9).unwrap() {
                let sphere_material: Arc<dyn Material<T>> = if choose_mat < 0.8 {
                    // diffuse
                    create_material!("lambertian", T, (rng.gen(), rng.gen(), rng.gen()))
                } else if choose_mat < 0.95 {
                    // metal
                    let fuzz = rng.gen_range(0.0..0.5);
                    create_material!("metal", T, (rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0)), fuzz)
                } else {
                    // glass
                    create_material!("dielectric", T, 1.5)
                };
                world.push(create_object!("sphere", T, (center_x, center_y, center_z), 0.2, sphere_material));
            }
        }
    }

    let material1 = create_material!("dielectric", T, 1.5);
    world.push(create_object!("sphere", T, (0., 1., 0.), 1.0, material1));

    let material2 = create_material!("lambertian", T, (0.4, 0.2, 0.1));
    world.push(create_object!("sphere", T, (-4., 1., 0.), 1.0, material2));

    let material3 = create_material!("metal", T, (0.7, 0.6, 0.5), 0.0);
    world.push(create_object!("sphere", T, (4., 1., 0.), 1.0, material3));

    world
}

fn main() {
    // World
    eprintln!("Creating world...");
    let world = random_scene();
    eprintln!("World created!");

    // Camera
    let cam = Camera::<f64>::new((13., 2., 3.), (0., 0., 0.), (0., 1., 0.), 20.0, ASPECT_RATIO, 0.1, 10.);
    
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    eprintln!("Rendering begins...");
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
                eprint!("{} jobs left\r", *thread_job_left);
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
    eprintln!("Rendering finishes...");

    eprintln!("Writing image...");
    write_image(&image);
    eprintln!("All completed!");
}
