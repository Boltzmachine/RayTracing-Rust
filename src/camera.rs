use num::Float;

use crate::vec3::*;
use crate::common::*;
use crate::ray::*;

pub struct Camera<T> 
where
    T: SVecElem
{
    pub origin: Point3<T>,
    pub lower_left_corner: Point3<T>,
    pub horizontal: Vec3<T>,
    pub vertical: Vec3<T>,

    pub u: Vec3<T>,
    pub v: Vec3<T>,
    pub w: Vec3<T>,
    pub lens_radius: T,
}

impl<T: SVecElem + Float> Camera<T> {
    pub fn new(
        lookfrom: (f64, f64, f64),
        lookat: (f64, f64, f64), 
        vup: (f64, f64, f64),
        vfov: f64, 
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {

        let lookfrom = Point3::<T>::new(lookfrom.0, lookfrom.1, lookfrom.2);
        let lookat = Point3::<T>::new(lookat.0, lookat.1, lookat.2);
        let vup = Vec3::<T>::new(vup.0, vup.1, vup.2);
        let focus_dist = T::from_f64(focus_dist).unwrap();
        
        let theta = vfov * std::f64::consts::PI / 180.;
        let h = (theta / 2.).tan();

        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).to_unit();
        let u = cross(&vup, &w).to_unit();
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u * T::from_f64(viewport_width).unwrap() * focus_dist;
        let vertical = v * T::from_f64(viewport_height).unwrap() * focus_dist;
        let lower_left_corner = origin - horizontal / T::from_f64(2.).unwrap() - vertical / T::from_f64(2.).unwrap() - w * focus_dist;

        let lens_radius = T::from_f64(aperture / 2.).unwrap();

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u, v, w,
            lens_radius,
        }
    }
}


impl<T> Camera<T> 
where
    T: SVecElem + Float,
{
    pub fn get_ray(&self, s: f64, t: f64) -> Ray<T> {
        let s = T::from_f64(s).unwrap();
        let t = T::from_f64(t).unwrap();

        let rd = random_in_unit_disk::<T>() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        let ray = Ray::<T> {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        };

        ray
    }
}