use std::ops::{Add, Sub, Mul, Div, Index, IndexMut, Neg, AddAssign};
use std::marker::Copy;
use num::{Float, FromPrimitive};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3<T> (pub T, pub T, pub T);

impl<T: Copy + Clone> Vec3<T> {
    pub fn x(&self) -> T { self.0 }
    pub fn y(&self) -> T { self.1 }
    pub fn z(&self) -> T { self.2 }
}

impl<T> Vec3<T> 
where
    T: FromPrimitive,
{
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(T::from_f64(x).unwrap(), T::from_f64(y).unwrap(), T::from_f64(z).unwrap())
    }
}

// Math operation
impl<T: Float> Add for Vec3<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }
}

impl<T: Float> Add<T> for Vec3<T> {
    type Output = Self;
    fn add(self, other: T) -> Self::Output {
        Self(self.x() + other, self.y() + other, self.z() + other )
    }
}

impl<T: Float> AddAssign for Vec3<T> {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0 + other.x();
        self.1 = self.1 + other.y();
        self.2 = self.2 + other.z();
    }
}

impl<T: Float> AddAssign<T> for Vec3<T> {
    fn add_assign(&mut self, other: T) {
        self.0 = self.0 + other;
        self.1 = self.1 + other;
        self.2 = self.2 + other;
    }
}

impl<T: Float> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}

impl<T: Float> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl<T: Float> Mul<Vec3<T>> for Vec3<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self(self.x() * other.x(), self.y() * other.y(), self.z() * other.z())
    }
}

impl<T: Float> Div<T> for Vec3<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl<T:Float> Div<Vec3<T>> for Vec3<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self(self.x() / other.x(), self.y() / other.y(), self.z() / other.z())
    }
}

impl <T: Float> Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.x(), -self.y(), -self.z())
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

impl<T: Float> Vec3<T> {
    pub fn length(&self) -> T {
        let val = self.x() * self.x() + self.y() * self.y() + self.z() * self.z();
        val.sqrt()
    }

    pub fn to_unit(self) -> Self {
        let len = self.length();
        Self(self.x() / len, self.y() / len, self.z() / len)
    }
}

pub fn dot<T: Float> (v1: Vec3<T>, v2: Vec3<T>) -> T {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

pub use Vec3 as Point3;
pub use Vec3 as Color3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _: Vec3<f32> = Vec3::new(1.0, 2.0, 3.0);
    }

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
    fn add_assign() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        v += 1.0;
        assert_eq!(v, Vec3(2.0, 3.0, 4.0));
        v += Vec3(1.0, 2.0, 3.0);
        assert_eq!(v, Vec3(3.0, 5.0, 7.0));
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

        assert_eq!(u, Vec3(2.0, 4.0, 6.0));
    }

    #[test]
    fn mul_f64_vector() {
        let v = Vec3(1.0, 2.0, 3.0);
        let u = Vec3(2.0, 4.0, 6.0);
        assert_eq!(v * u, Vec3(2.0, 8.0, 18.0));
    }

    #[test]
    fn div_float_scalar() {
        let v = Vec3(1.0, 2.0, 3.0);
        let u = v / 2.0;
        assert_eq!(u, Vec3(0.5, 1.0, 1.5));

        let v = Vec3::<f32>::new(1.0, 2.0, 3.0);
        let u = v / 2.0;
        assert_eq!(u, Vec3::<f32>::new(0.5, 1.0, 1.5));
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