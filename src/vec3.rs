use std::ops::{Sub, Div, Index, IndexMut};
use std::cmp::Eq;

pub struct Vec3 (pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn x(&self) -> f64 { self.0 }
    pub fn y(&self) -> f64 { self.1 }
    pub fn z(&self) -> f64 { self.2 }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Eq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        let square = self.x() * self.x() + self.y() * self.y() + self.z() * self.z();
        square.sqrt()
    }

    pub fn to_unit(&self) -> Self {
        let k = 1.0 / self.length();
        Self(self.x() * k, self.y() * k, self.z() * k)
    }
}

pub use Vec3 as Point3;
pub use Vec3 as Color3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coorditates() {
        let v = Vec3(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn alias() {
        let p = Point3(1.0, 2.0, 3.0);
        assert_eq!(p.x(), 1.0);

        let c = Color3(1.0, 2.0, 3.0);
        assert_eq!(c.x(), 1.0);
    }

    #[test]
    fn subtract() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, 5.0, 6.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x(), -3.0);
        assert_eq!(v3.y(), -3.0);
        assert_eq!(v3.z(), -3.0);
    }

    #[test]
    fn index() {
        let v = Vec3(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn index_mut() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        v[0] = 4.0;
        assert_eq!(v[0], 4.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn div() {
        let v = Vec3(1.0, 2.0, 3.0);
        let u = v / 2.0;
    }

    #[test]
    fn eq() {
        assert_eq!(
            Vec3(1.0, 2.0, 3.0),
            Vec3(1.0, 2.0, 3.0)            
        );
        assert_ne!(
            Vec3(1.0, 2.0, 3.0),
            Vec3(1.0, 2.0, 4.0)
        );
    }
    
    #[test]
    fn length() {
        let v = Vec3(1.0, 2.0, 2.0);
        assert_eq!(v.length(), 3.);
    }

    #[test]
    fn to_unit() {
        let v = Vec3(1.0, 2.0, 2.0);
        assert_eq!(v.to_unit().length(), 1.)
    }
 }