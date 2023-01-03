use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct vec3<T>(pub T, pub T, pub T);

impl<T: Clone> From<T> for vec3<T> {
    fn from(value: T) -> Self {
        Self(value.clone(), value.clone(), value)
    }
}

impl<T> From<(T, T, T)> for vec3<T> {
    fn from(value: (T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl<T: Clone> From<(&T, &T, &T)> for vec3<T> {
    fn from(value: (&T, &T, &T)) -> Self {
        Self::new(value.0.clone(), value.1.clone(), value.2.clone())
    }
}

impl<T: TBytes + Copy> Copy for vec3<T> {}

impl<T> vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> &T {
        &self.0
    }

    pub fn y(&self) -> &T {
        &self.1
    }

    pub fn z(&self) -> &T {
        &self.2
    }

    pub fn xy(&self) -> (&T, &T) {
        (self.x(), self.y())
    }
    pub fn yx(&self) -> (&T, &T) {
        (self.y(), self.x())
    }

    pub fn xz(&self) -> (&T, &T) {
        (self.x(), self.z())
    }
    pub fn zx(&self) -> (&T, &T) {
        (self.z(), self.x())
    }

    pub fn yz(&self) -> (&T, &T) {
        (self.y(), self.z())
    }
    pub fn zy(&self) -> (&T, &T) {
        (self.z(), self.y())
    }

    pub fn xyz(&self) -> (&T, &T, &T) {
        (self.x(), self.y(), self.z())
    }
    pub fn zxy(&self) -> (&T, &T, &T) {
        (self.z(), self.x(), self.y())
    }
    pub fn yzx(&self) -> (&T, &T, &T) {
        (self.y(), self.z(), self.x())
    }

    pub fn yxz(&self) -> (&T, &T, &T) {
        (self.y(), self.x(), self.z())
    }
    pub fn zyx(&self) -> (&T, &T, &T) {
        (self.z(), self.y(), self.x())
    }
    pub fn xzy(&self) -> (&T, &T, &T) {
        (self.x(), self.z(), self.y())
    }

    pub fn r(&self) -> &T {
        self.x()
    }
    pub fn g(&self) -> &T {
        self.y()
    }
    pub fn b(&self) -> &T {
        self.z()
    }

    pub fn rg(&self) -> (&T, &T) {
        self.xy()
    }
    pub fn gr(&self) -> (&T, &T) {
        self.yx()
    }

    pub fn rb(&self) -> (&T, &T) {
        self.xz()
    }
    pub fn br(&self) -> (&T, &T) {
        self.zx()
    }

    pub fn rgb(&self) -> (&T, &T, &T) {
        self.xyz()
    }
    pub fn brg(&self) -> (&T, &T, &T) {
        self.zxy()
    }
    pub fn gbr(&self) -> (&T, &T, &T) {
        self.yzx()
    }

    pub fn grb(&self) -> (&T, &T, &T) {
        self.yxz()
    }
    pub fn bgr(&self) -> (&T, &T, &T) {
        self.zyx()
    }
    pub fn rbg(&self) -> (&T, &T, &T) {
        self.xzy()
    }
}

impl<T: TBytes + Clone> Clone for vec3<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

impl<T: TBytes + std::fmt::Debug> std::fmt::Debug for vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("vec3")
            .field("x", &self.0)
            .field("y", &self.1)
            .field("z", &self.2)
            .finish()
    }
}

impl<T: Add<Output = T>> Add for vec3<T> {
    type Output = vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: Sub<Output = T>> Sub for vec3<T> {
    type Output = vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: Mul<Output = T>> Mul for vec3<T> {
    type Output = vec3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl<T: Div<Output = T>> Div for vec3<T> {
    type Output = vec3<T>;

    fn div(self, rhs: Self) -> Self::Output {
        vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl<T: AddAssign> AddAssign for vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl<T: SubAssign> SubAssign for vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl<T: MulAssign> MulAssign for vec3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl<T: DivAssign> DivAssign for vec3<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}
