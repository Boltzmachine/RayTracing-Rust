mod vec3;
use vec3::*;

struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn general() {
        let ray = Ray(
            Point3(1.0, 2.0, 3.0),
            Vec3(1.0, 2.0, 3.0),
        );

        assert_eq!(ray.at(0.0), Point3(1.0, 2.0, 3.0));
        assert_eq!(ray.at(1.0), Point3(2.0, 4.0, 6.0));
    }
}
