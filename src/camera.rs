use crate::vec3::*;
use crate::common::*;
use crate::ray::*;
use num::{FromPrimitive, Float};

pub struct Camera<T> 
where
    T: FromPrimitive
{
    pub origin: Point3<T>,
    pub lower_left_corner: Point3<T>,
    pub horizontal: Vec3<T>,
    pub vertical: Vec3<T>,
}

impl<T> Default for Camera<T> 
where
    T: FromPrimitive + Float
{
    fn default () -> Self {
        let origin = Point3::<T>::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::<T>::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::<T>::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner = origin - horizontal / T::from_f64(2.0).unwrap() - vertical / T::from_f64(2.0).unwrap() - Vec3::<T>::new(0., 0., FOCAL_LENGTH);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}

impl<T> Camera<T> 
where
    T: FromPrimitive + Float,
{
    pub fn get_ray(&self, u: f64, v: f64) -> Ray<T> {
        if u == 0.5 && v == 0.5 {
            eprintln!("ray: {}", u);
    }
        let u = T::from_f64(u).unwrap();
        let v = T::from_f64(v).unwrap();

        let ray = Ray::<T> {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        };

        ray
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn general() {
        let camera = Camera::<f64>::default();

        assert_eq!(camera.origin, Point3(0.0, 0.0, 0.0));
        assert_eq!(camera.horizontal, Vec3(VIEWPORT_WIDTH, 0.0, 0.0));
        assert_eq!(camera.vertical, Vec3(0.0, VIEWPORT_HEIGHT, 0.0));
        assert_eq!(camera.lower_left_corner, Point3(-VIEWPORT_WIDTH / 2., -VIEWPORT_HEIGHT / 2., -FOCAL_LENGTH));
    }
}