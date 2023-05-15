use crate::vec3::*;
use crate::common::SVecElem;
use std::cmp::min;
use num::{Float, abs};


pub struct Ray<T> 
where
    T: SVecElem
{
    pub origin: Point3<T>,
    pub direction: Vec3<T>,
}

impl<T: SVecElem> Ray<T> {
    pub fn at(&self, t: T) -> Point3<T> {
        self.origin + self.direction * t
    }
}

pub fn reflect<T: SVecElem>(v: &Vec3<T>, n: &Vec3<T>) -> Vec3<T> {
    *v - *n * dot(v, n) * T::from_i8(2).unwrap()
}

pub fn refract <T: SVecElem + Float>(uv: &Vec3<T>, n: &Vec3<T>, etai_over_etat: T) -> Vec3<T> {
    let cos_theta = dot(&-*uv, n).min(T::from_i8(1).unwrap());

    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * -T::sqrt(T::from_i8(1).unwrap() - dot(&r_out_perp, &r_out_perp));
    r_out_perp + r_out_parallel
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn general() {
        let ray = Ray {
            origin: Point3(1.0, 2.0, 3.0),
            direction: Vec3(1.0, 2.0, 3.0),
        };

        assert_eq!(ray.at(0.0), Point3(1.0, 2.0, 3.0));
        assert_eq!(ray.at(1.0), Point3(2.0, 4.0, 6.0));
    }
}
