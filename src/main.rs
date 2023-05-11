mod vec3;
use vec3::*;

macro_rules! float_to_rgb_scalar {
    ($x:expr) => {
        (255.999 * $x) as u8
    };
}

macro_rules! float_to_rgb_vec {
    ($r:expr, $g:expr, $b:expr) => {
        (float_to_rgb_scalar!($r), float_to_rgb_scalar!($g), float_to_rgb_scalar!($b))
    };
}

fn write_color(pixel_color: Color3) {
    let (r, g, b) = float_to_rgb_vec!(pixel_color.x(), pixel_color.y(), pixel_color.z());
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
