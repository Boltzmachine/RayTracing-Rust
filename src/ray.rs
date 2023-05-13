use crate::vec3::*;
use num::Float;

#[derive(Debug, Copy, Clone)]
pub struct Ray<T> 
where
    T: Float
{
    pub origin: Point3<T>,
    pub direction: Vec3<T>,
}

impl<T: Float> Ray<T> {
    pub fn at(&self, t: T) -> Point3<T> {
        self.origin + self.direction * t
    }
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
