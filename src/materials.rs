use num::Float;

use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;
use crate::common::{SVecElem, random_in_unit_sphere};

pub trait Material<T: SVecElem>: Send + Sync {
    fn scatter(&self, ray_in: &Ray<T>, rec: &HitRecord<T>) -> Option<(Vec3<T>, Ray<T>)>;
}
pub struct Lambertian<T: SVecElem + Float> {
    pub albedo: Color3<T>,
}

impl<T> Material<T> for Lambertian<T>
where
    T: SVecElem + Float,
{
    fn scatter(&self, ray_in: &Ray<T>, rec: &HitRecord<T>) -> Option<(Color3<T>, Ray<T>)> {
        let mut scatter_direction: Vec3<T> = rec.normal + random_in_unit_sphere();

        if scatter_direction.is_close(T::from_f64(0.).unwrap()) {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::<T> {
            origin: rec.p,
            direction: scatter_direction,
        };
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}