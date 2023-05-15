use crate::common::SVecElem;
use crate::vec3::*;
use crate::ray::*;
use crate::materials::*;

use std::sync::Arc;
use num::Float;

pub struct HitRecord<'a, T: SVecElem> {
    pub p: Point3<T>,
    pub normal: Vec3<T>,
    pub material: Arc<dyn Material<T> + 'a>,
    pub t: T,
    pub front_face: bool,
}

pub trait Hittable<T>
where
    T: SVecElem,
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>>;
}

pub struct Sphere<'a, T: SVecElem + Float> {
    pub center: Point3<T>,
    pub radius: T,
    pub material: Arc<dyn Material<T> + 'a>
}

impl<T> Hittable<T> for Sphere<'_, T>
where
    T: SVecElem + Float,
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>> {
        let oc = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;

        let discriminant: T = half_b * half_b - a * c;
        if discriminant < T::from_f64(0.).unwrap() {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let p = ray.at(root);
        let mut normal = (p - self.center) / self.radius;
        let front_face = dot(&ray.direction, &normal) < T::from_f64(0.).unwrap();

        if !front_face { normal = - normal; }

        Some(HitRecord {
            t: root,
            p,
            material: Arc::clone(&self.material),
            normal,
            front_face,
        })
    }
}
