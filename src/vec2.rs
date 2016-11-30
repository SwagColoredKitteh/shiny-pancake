use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec2(pub f64, pub f64);

impl Vec2 {
    #[inline]
    pub fn zero() -> Vec2 {
        Vec2(0., 0.)
    }

    #[inline]
    pub fn len_sq(self) -> f64 {
        self.0.powi(2) + self.1.powi(2)
    }

    #[inline]
    pub fn len(self) -> f64 {
        self.len_sq().sqrt()
    }
    
    #[inline]
    pub fn norm(self) -> Vec2 {
        let ilen = 1. / self.len();
        self * ilen
    }

    #[inline]
    pub fn rotate(self, angle: f64) -> Vec2 {
        let x = self.0;
        let y = self.1;
        let c = angle.cos();
        let s = angle.sin();
        Vec2(x * c - y * s, x * s + y * c)
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, other: Vec2) -> Vec2 {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = f64;

    #[inline]
    fn mul(self, other: Vec2) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, other: f64) -> Vec2 {
        Vec2(self.0 * other, self.1 * other)
    }
}
