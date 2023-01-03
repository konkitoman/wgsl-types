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

impl<T: Clone> From<(&mut T, &mut T, &mut T)> for vec3<T> {
    fn from(value: (&mut T, &mut T, &mut T)) -> Self {
        Self::new(value.0.clone(), value.1.clone(), value.2.clone())
    }
}

impl<T: TBytes + Copy> Copy for vec3<T> {}

impl<T> vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }

    pub fn x(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn y(&mut self) -> &mut T {
        &mut self.1
    }

    pub fn z(&mut self) -> &mut T {
        &mut self.2
    }

    pub fn xy(&mut self) -> (&mut T, &mut T) {
        (&mut self.0, &mut self.1)
    }
    pub fn yx(&mut self) -> (&mut T, &mut T) {
        (&mut self.1, &mut self.0)
    }

    pub fn xz(&mut self) -> (&mut T, &mut T) {
        (&mut self.0, &mut self.2)
    }
    pub fn zx(&mut self) -> (&mut T, &mut T) {
        (&mut self.2, &mut self.0)
    }

    pub fn yz(&mut self) -> (&mut T, &mut T) {
        (&mut self.1, &mut self.2)
    }
    pub fn zy(&mut self) -> (&mut T, &mut T) {
        (&mut self.2, &mut self.1)
    }

    pub fn xyz(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.0, &mut self.1, &mut self.2)
    }
    pub fn zxy(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.2, &mut self.0, &mut self.1)
    }
    pub fn yzx(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.1, &mut self.2, &mut self.0)
    }

    pub fn yxz(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.1, &mut self.0, &mut self.2)
    }
    pub fn zyx(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.2, &mut self.1, &mut self.0)
    }
    pub fn xzy(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.0, &mut self.2, &mut self.1)
    }

    pub fn r(&mut self) -> &mut T {
        self.x()
    }
    pub fn g(&mut self) -> &mut T {
        self.y()
    }
    pub fn b(&mut self) -> &mut T {
        self.z()
    }

    pub fn rg(&mut self) -> (&mut T, &mut T) {
        self.xy()
    }
    pub fn gr(&mut self) -> (&mut T, &mut T) {
        self.yx()
    }

    pub fn rb(&mut self) -> (&mut T, &mut T) {
        self.xz()
    }
    pub fn br(&mut self) -> (&mut T, &mut T) {
        self.zx()
    }

    pub fn rgb(&mut self) -> (&mut T, &mut T, &mut T) {
        self.xyz()
    }
    pub fn brg(&mut self) -> (&mut T, &mut T, &mut T) {
        self.zxy()
    }
    pub fn gbr(&mut self) -> (&mut T, &mut T, &mut T) {
        self.yzx()
    }

    pub fn grb(&mut self) -> (&mut T, &mut T, &mut T) {
        self.yxz()
    }
    pub fn bgr(&mut self) -> (&mut T, &mut T, &mut T) {
        self.zyx()
    }
    pub fn rbg(&mut self) -> (&mut T, &mut T, &mut T) {
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
