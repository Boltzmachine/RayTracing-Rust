use num::Float;
use rand::Rng;

use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;
use crate::common::{SVecElem, random_in_unit_sphere};

pub trait Material<T: SVecElem>: Send + Sync {
    fn scatter(&self, ray_in: &Ray<T>, rec: &HitRecord<T>) -> Option<(Color3<T>, Ray<T>)>;
}
pub struct Lambertian<T: SVecElem + Float> {
    pub albedo: Color3<T>,
}

impl<T> Material<T> for Lambertian<T>
where
    T: SVecElem + Float,
{
    fn scatter(&self, _ray_in: &Ray<T>, rec: &HitRecord<T>) -> Option<(Color3<T>, Ray<T>)> {
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

pub struct Metal<T: SVecElem + Float> {
    pub albedo: Color3<T>,
    pub fuzz: T,
}

impl<T> Material<T> for Metal<T>
where
    T: SVecElem + Float,
{
    fn scatter(&self, ray_in: &Ray<T>, rec: &HitRecord<T>) -> Option<(Color3<T>, Ray<T>)> {
        let reflected = reflect(&ray_in.direction.to_unit(), &rec.normal);
        let scattered = Ray::<T> {
            origin: rec.p,
            direction: reflected + random_in_unit_sphere() * self.fuzz,
        };
        let attenuation = self.albedo;

        if dot(&scattered.direction, &rec.normal) > T::from_i8(0).unwrap() {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric<T: SVecElem + Float> {
    pub ir: T,
}

impl<T: SVecElem + Float> Dielectric<T> {
    fn reflectance(cosine: T, ref_idx: T) -> T {
        let r0 = (T::from_i8(1).unwrap() - ref_idx) / (T::from_i8(1).unwrap() + ref_idx);
        let r0 = r0 * r0;
        r0 + (T::from_i8(1).unwrap() - r0) * T::powi(T::from_i8(1).unwrap() - cosine, 5)
    }
} 

impl<T> Material<T> for Dielectric<T>
where
    T: SVecElem + Float,
{
    fn scatter(&self, ray_in: &Ray<T>, rec: &HitRecord<T>) -> Option<(Color3<T>, Ray<T>)> {
        let attenuation = Color3::new(1., 1.,1.);

        let refraction_ratio = if rec.front_face {
            T::from_f64(1.).unwrap() / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction.to_unit();

        let cos_theta = dot(&-unit_direction, &rec.normal).min(T::from_i8(1).unwrap());
        let sin_theta = T::sqrt(T::from_i8(1).unwrap() - cos_theta * cos_theta);
        let cannot_refract = refraction_ratio * sin_theta > T::from_i8(1).unwrap();
        
        let direction = if cannot_refract || Dielectric::<T>::reflectance(cos_theta, refraction_ratio).to_f64().unwrap() > rand::thread_rng().gen() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scattered = Ray::<T> {
            origin: rec.p,
            direction: direction,
        };
        Some((attenuation, scattered))
    }
}