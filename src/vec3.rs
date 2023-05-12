use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};
use std::marker::Copy;
use num_traits::real::Real;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3<T> (pub T, pub T, pub T);

impl<T: Copy + Clone> Vec3<T> {
    pub fn x(&self) -> T { self.0 }
    pub fn y(&self) -> T { self.1 }
    pub fn z(&self) -> T { self.2 }
}


// Math operation
impl <T: Real> Add for Vec3<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }
}

impl<T: Real> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}

impl <T: Real> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl <T: Real> Mul<Vec3<T>> for Vec3<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self(self.x() * other.x(), self.y() * other.y(), self.z() * other.z())
    }
}

impl <T: Real> Mul<Vec3<T>> for T {
    type Output = Vec3<T>;
    fn mul(self, other: Vec3<T>) -> Self::Output {
        Vec3(self * other.x(), self * other.y(), self * other.z())
    }
}

impl<T: Real> Div<T> for Vec3<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl <T:Real> Div<Vec3<T>> for Vec3<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self(self.x() / other.x(), self.y() / other.y(), self.z() / other.z())
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

impl<T: Real> Vec3<T> {
    pub fn length(&self) -> T {
        let val = self.x() * self.x() + self.y() * self.y() + self.z() * self.z();
        val.sqrt()
    }

    pub fn to_unit(&self) -> Self {
        let len = self.length();
        Self(self.x() / len, self.y() / len, self.z() / len)
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
    fn add_f64() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x(), 5.0);
        assert_eq!(v3.y(), 7.0);
        assert_eq!(v3.z(), 9.0);
    }

    #[test]
    fn subtract_f64() {
        let v1 = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(4.0, 5.0, 6.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x(), -3.0);
        assert_eq!(v3.y(), -3.0);
        assert_eq!(v3.z(), -3.0);
    }
    
    #[test]
    fn mul_f64_scalar() {
        let v = Vec3(1.0, 2.0, 3.0);
        let u = v * 2.0;
        let w = 3.0 * v;

        assert_eq!(u, Vec3(2.0, 4.0, 6.0));
        assert_eq!(w, Vec3(3.0, 6.0, 9.0));

    }

    #[test]
    fn mul_f64_vector() {
        let v = Vec3(1.0, 2.0, 3.0);
        let u = Vec3(2.0, 4.0, 6.0);
        assert_eq!(v * u, Vec3(2.0, 8.0, 18.0));
    }

    #[test]
    fn div_f64_scalar() {
        let v = Vec3(1.0, 2.0, 3.0);
        let u = v / 2.0;
        assert_eq!(u, Vec3(0.5, 1.0, 1.5));
    }

    #[test]
    fn div_f64_vector() {
        let v = Vec3(1.0, 2.0, 6.0);
        let u = Vec3(2.0, 4.0, 6.0);
        assert_eq!(v / u, Vec3(0.5, 0.5, 1.0));
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