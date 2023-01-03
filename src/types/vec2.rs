use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct vec2<T>(pub T, pub T);

impl<T: Clone + TBytes> From<T> for vec2<T> {
    fn from(value: T) -> Self {
        Self::new(value.clone(), value)
    }
}

impl<T: TBytes> From<(T, T)> for vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T: Clone> From<(&T, &T)> for vec2<T> {
    fn from(value: (&T, &T)) -> Self {
        Self::new(value.0.clone(), value.1.clone())
    }
}

impl<T: TBytes + Copy> Copy for vec2<T> {}

impl<T> vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> &T {
        &self.0
    }

    pub fn y(&self) -> &T {
        &self.1
    }

    pub fn xy(&self) -> (&T, &T) {
        (self.x(), self.y())
    }

    pub fn yx(&self) -> (&T, &T) {
        (self.y(), self.x())
    }

    pub fn r(&self) -> &T {
        self.x()
    }

    pub fn g(&self) -> &T {
        self.y()
    }

    pub fn rg(&self) -> (&T, &T) {
        self.xy()
    }

    pub fn gr(&self) -> (&T, &T) {
        self.yx()
    }
}

impl<T: TBytes + PartialEq> PartialEq for vec2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<T: TBytes + Clone> Clone for vec2<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<T: TBytes + std::fmt::Debug> std::fmt::Debug for vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("vec2")
            .field("x", &self.0)
            .field("y", &self.1)
            .finish()
    }
}

impl<T: Add<Output = T>> Add for vec2<T> {
    type Output = vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Sub<Output = T>> Sub for vec2<T> {
    type Output = vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Mul<Output = T>> Mul for vec2<T> {
    type Output = vec2<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl<T: Div<Output = T>> Div for vec2<T> {
    type Output = vec2<T>;

    fn div(self, rhs: Self) -> Self::Output {
        vec2(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl<T: AddAssign> AddAssign for vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: SubAssign> SubAssign for vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1
    }
}

impl<T: MulAssign> MulAssign for vec2<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
    }
}

impl<T: DivAssign> DivAssign for vec2<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
    }
}
