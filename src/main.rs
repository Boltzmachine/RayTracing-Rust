mod vec3;
mod ray;
use vec3::*;
use ray::*;
use num_traits::cast::ToPrimitive;


fn float_to_rgb_vec<T: ToPrimitive>(r: T, g: T, b: T) -> (u8, u8, u8) {
    let r = (255.999 * r.to_f64().unwrap()) as u8;
    let g = (255.999 * g.to_f64().unwrap()) as u8;
    let b = (255.999 * b.to_f64().unwrap()) as u8;
    (r, g, b)
}

fn write_color<T: Copy + ToPrimitive>(pixel_color: Color3<T>) {
    let (r, g, b) = float_to_rgb_vec(pixel_color.x(), pixel_color.y(), pixel_color.z());
    println!("{} {} {}", r, g, b);
}

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("scanlines remaining: {}\r", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let pixel_color = Color3(r, g, b);
            
            write_color(pixel_color)
        }
    }

}
