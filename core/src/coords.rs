use num_traits::{Num, One, Zero};
use std::ops::{Mul, Add, Sub};

#[derive(Debug, Clone)]
pub struct Vec4<T: Num + Default + PartialEq> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec4<T>
where
    T: Num + Default + PartialEq,
{
    pub fn new() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            w: Default::default(),
        }
    }

    pub fn from(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T> Zero for Vec4<T>
where
    T: Num + Default + PartialEq,
{
    fn zero() -> Self {
        Vec4::from(Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero())
    }

    fn set_zero(&mut self) {
        self.x = Zero::zero();
        self.y = Zero::zero();
        self.z = Zero::zero();
        self.w = Zero::zero();
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero()
    }
}

impl<T> One for Vec4<T>
where
    T: Num + Default + PartialEq,
{
    fn one() -> Self {
        Self {
            x: One::one(),
            y: One::one(),
            z: One::one(),
            w: One::one(),
        }
    }

    fn set_one(&mut self) {
        self.x = One::one();
        self.y = One::one();
        self.z = One::one();
        self.w = One::one();
    }

    fn is_one(&self) -> bool {
        self.x.is_one() && self.y.is_one() && self.z.is_one() && self.w.is_one()
    }
}

impl<T> Mul for Vec4<T>
    where
        T: Num + Default + PartialEq,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl<T> Add for Vec4<T>
    where
        T: Num + Default + PartialEq,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T> Sub for Vec4<T>
where
    T: Num + Default + PartialEq,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl <T> PartialEq for Vec4<T> where T: Num + Default + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.x, &other.x) &&
        PartialEq::eq(&self.y, &other.y) &&
        PartialEq::eq(&self.z, &other.z) &&
        PartialEq::eq(&self.w, &other.w)
    }
}

#[cfg(test)]
mod tests {
    use crate::coords::Vec4;
    use num_traits::One;
    use num_traits::Zero;

    #[test]
    fn vec4_new() {
        let v: Vec4<f64> = Vec4::new();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
        assert_eq!(v.w, 0.0);
    }

    #[test]
    fn vec4_from() {
        let v: Vec4<f64> = Vec4::from(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.w, 4.0);
    }

    #[test]
    fn vec4_zero() {
        let v: Vec4<f64> = Vec4::zero();
        assert_eq!(v.is_zero(), true);
        assert_eq!(v.is_one(), false);
    }

    #[test]
    fn vec4_one() {
        let v: Vec4<f64> = Vec4::one();
        assert_eq!(v.is_zero(), false);
        assert_eq!(v.is_one(), true);
    }
}
