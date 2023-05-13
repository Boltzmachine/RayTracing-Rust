// IMAGE
pub const ASPECT_RATIO: f64 = 16. / 9.0;
pub const IMAGE_WIDTH: u32 = 400;
pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
pub const AA_SAMPLES: u32 = 100;

// Camera
pub const VIEWPORT_HEIGHT: f64 = 2.;
pub const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
pub const FOCAL_LENGTH: f64 = 1.;