use crate::vec3::*;
use crate::ray::*;
use num::Float;

pub struct HitRecord<T> {
    pub p: Point3<T>,
    pub normal: Vec3<T>,
    pub t: T,
    pub front_face: bool,
}

pub trait Hittable<T> 
where
    T: Float,
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>>;
}

#[derive(Debug)]
pub struct Sphere<T> {
    pub center: Point3<T>,
    pub radius: T,
}

impl<T> Hittable<T> for Sphere<T>
where
    T: Float,
    f64: Into<T>
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>> {
        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction);
        let half_b = dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius * self.radius;

        let discriminant: T = half_b * half_b - a * c;
        if discriminant < 0.0.into() {
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
        let front_face = dot(ray.direction, normal) < 0.0.into();

        if !front_face { normal = - normal; }

        Some(HitRecord {
            t: root,
            p,
            normal,
            front_face,
        })
    }
}
