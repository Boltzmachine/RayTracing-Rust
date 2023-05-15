use crate::common::SVecElem;
use crate::ray::*;
use crate::hittable::*;


pub type HittableList<'a, T> = Vec<Box<dyn Hittable<T> + 'a + Send + Sync>>;

impl<T> Hittable<T> for HittableList<'_, T>
where
    T: SVecElem,
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