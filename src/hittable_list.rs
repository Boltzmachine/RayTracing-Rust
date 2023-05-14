use crate::ray::*;
use crate::hittable::*;
use num::Float;

pub type HittableList<T> = Vec<Box<dyn Hittable<T> + Send + Sync>>;

impl<T> Hittable<T> for HittableList<T>
where
    T: Float,
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>> {
        let mut closest_so_far = t_max;
        let mut result = None;

        for object in self {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                result = Some(hit_record);
            }
        }
        result
    }
}