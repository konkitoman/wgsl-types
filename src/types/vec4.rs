use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct vec4<T>(pub T, pub T, pub T, pub T);

impl<T: Clone> From<T> for vec4<T> {
    fn from(value: T) -> Self {
        Self::new(value.clone(), value.clone(), value.clone(), value)
    }
}

impl<T> From<(T, T, T, T)> for vec4<T> {
    fn from(value: (T, T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl<T: Clone> From<(&T, &T, &T, &T)> for vec4<T> {
    fn from(value: (&T, &T, &T, &T)) -> Self {
        Self::new(
            value.0.clone(),
            value.1.clone(),
            value.2.clone(),
            value.3.clone(),
        )
    }
}

impl<T: Clone> From<(&mut T, &mut T, &mut T, &mut T)> for vec4<T> {
    fn from(value: (&mut T, &mut T, &mut T, &mut T)) -> Self {
        Self::new(
            value.0.clone(),
            value.1.clone(),
            value.2.clone(),
            value.3.clone(),
        )
    }
}

impl<T: TBytes + Copy> Copy for vec4<T> {}

impl<T> vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self(x, y, z, w)
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

    pub fn w(&self) -> &T {
        &self.3
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

    pub fn xw(&self) -> (&T, &T) {
        (self.x(), self.w())
    }
    pub fn wx(&self) -> (&T, &T) {
        (self.w(), self.x())
    }

    pub fn zw(&self) -> (&T, &T) {
        (self.z(), self.w())
    }
    pub fn wz(&self) -> (&T, &T) {
        (self.w(), self.z())
    }

    pub fn xyw(&self) -> (&T, &T, &T) {
        (self.x(), self.y(), self.w())
    }
    pub fn wxy(&self) -> (&T, &T, &T) {
        (self.w(), self.x(), self.y())
    }
    pub fn ywx(&self) -> (&T, &T, &T) {
        (self.y(), self.w(), self.x())
    }

    pub fn yxw(&self) -> (&T, &T, &T) {
        (self.y(), self.x(), self.w())
    }
    pub fn wyx(&self) -> (&T, &T, &T) {
        (self.w(), self.y(), self.x())
    }
    pub fn xwy(&self) -> (&T, &T, &T) {
        (self.x(), self.w(), self.y())
    }

    pub fn xwz(&self) -> (&T, &T, &T) {
        (self.x(), self.w(), self.z())
    }
    pub fn zxw(&self) -> (&T, &T, &T) {
        (self.z(), self.x(), self.w())
    }
    pub fn wzx(&self) -> (&T, &T, &T) {
        (self.w(), self.z(), self.x())
    }

    pub fn wxz(&self) -> (&T, &T, &T) {
        (self.w(), self.x(), self.z())
    }
    pub fn zwx(&self) -> (&T, &T, &T) {
        (self.z(), self.w(), self.x())
    }
    pub fn xzw(&self) -> (&T, &T, &T) {
        (self.x(), self.z(), self.w())
    }

    pub fn xyzw(&self) -> (&T, &T, &T, &T) {
        (self.x(), self.y(), self.z(), self.w())
    }
    pub fn wxyz(&self) -> (&T, &T, &T, &T) {
        (self.w(), self.x(), self.y(), self.z())
    }
    pub fn zwxy(&self) -> (&T, &T, &T, &T) {
        (self.z(), self.w(), self.x(), self.y())
    }
    pub fn yzwx(&self) -> (&T, &T, &T, &T) {
        (self.y(), self.z(), self.w(), self.x())
    }

    pub fn yxzw(&self) -> (&T, &T, &T, &T) {
        (self.y(), self.x(), self.z(), self.w())
    }
    pub fn wyxz(&self) -> (&T, &T, &T, &T) {
        (self.w(), self.y(), self.x(), self.z())
    }
    pub fn zwyx(&self) -> (&T, &T, &T, &T) {
        (self.z(), self.w(), self.y(), self.x())
    }
    pub fn xzwy(&self) -> (&T, &T, &T, &T) {
        (self.x(), self.z(), self.w(), self.y())
    }

    pub fn xywz(&self) -> (&T, &T, &T, &T) {
        (self.x(), self.y(), self.w(), self.z())
    }
    pub fn zxyw(&self) -> (&T, &T, &T, &T) {
        (self.z(), self.x(), self.y(), self.w())
    }
    pub fn wzxy(&self) -> (&T, &T, &T, &T) {
        (self.w(), self.z(), self.x(), self.y())
    }
    pub fn ywzx(&self) -> (&T, &T, &T, &T) {
        (self.y(), self.w(), self.z(), self.x())
    }

    pub fn xzyw(&self) -> (&T, &T, &T, &T) {
        (self.x(), self.z(), self.y(), self.w())
    }
    pub fn wxzy(&self) -> (&T, &T, &T, &T) {
        (self.w(), self.x(), self.z(), self.y())
    }
    pub fn ywxz(&self) -> (&T, &T, &T, &T) {
        (self.y(), self.w(), self.x(), self.z())
    }
    pub fn zywx(&self) -> (&T, &T, &T, &T) {
        (self.z(), self.y(), self.w(), self.x())
    }

    pub fn wyzx(&self) -> (&T, &T, &T, &T) {
        (self.w(), self.y(), self.z(), self.x())
    }
    pub fn xwyz(&self) -> (&T, &T, &T, &T) {
        (self.x(), self.w(), self.y(), self.z())
    }
    pub fn zxwy(&self) -> (&T, &T, &T, &T) {
        (self.z(), self.x(), self.w(), self.y())
    }
    pub fn yzxw(&self) -> (&T, &T, &T, &T) {
        (self.y(), self.z(), self.x(), self.w())
    }

    pub fn xwzy(&self) -> (&T, &T, &T, &T) {
        (self.x(), self.w(), self.z(), self.y())
    }
    pub fn yxwz(&self) -> (&T, &T, &T, &T) {
        (self.y(), self.x(), self.w(), self.z())
    }
    pub fn zyxw(&self) -> (&T, &T, &T, &T) {
        (self.z(), self.y(), self.x(), self.w())
    }
    pub fn wzyx(&self) -> (&T, &T, &T, &T) {
        (self.w(), self.z(), self.y(), self.x())
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
    pub fn a(&self) -> &T {
        self.w()
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

    pub fn rga(&self) -> (&T, &T, &T) {
        self.xyw()
    }
    pub fn arg(&self) -> (&T, &T, &T) {
        self.wxy()
    }
    pub fn gar(&self) -> (&T, &T, &T) {
        self.ywx()
    }

    pub fn gra(&self) -> (&T, &T, &T) {
        self.yxw()
    }
    pub fn agr(&self) -> (&T, &T, &T) {
        self.wyx()
    }
    pub fn rag(&self) -> (&T, &T, &T) {
        self.xwy()
    }

    pub fn rab(&self) -> (&T, &T, &T) {
        self.xwz()
    }
    pub fn bra(&self) -> (&T, &T, &T) {
        self.zxw()
    }
    pub fn abr(&self) -> (&T, &T, &T) {
        self.wzx()
    }

    pub fn rgba(&self) -> (&T, &T, &T, &T) {
        self.xyzw()
    }
    pub fn argb(&self) -> (&T, &T, &T, &T) {
        self.wxyz()
    }
    pub fn barg(&self) -> (&T, &T, &T, &T) {
        self.zwxy()
    }
    pub fn gbar(&self) -> (&T, &T, &T, &T) {
        self.yzwx()
    }

    pub fn grba(&self) -> (&T, &T, &T, &T) {
        self.yxzw()
    }
    pub fn agrb(&self) -> (&T, &T, &T, &T) {
        self.wyxz()
    }
    pub fn bagr(&self) -> (&T, &T, &T, &T) {
        self.zwyx()
    }
    pub fn rbag(&self) -> (&T, &T, &T, &T) {
        self.xzwy()
    }

    pub fn rgab(&self) -> (&T, &T, &T, &T) {
        self.xywz()
    }
    pub fn brga(&self) -> (&T, &T, &T, &T) {
        self.zxyw()
    }
    pub fn abrg(&self) -> (&T, &T, &T, &T) {
        self.wzxy()
    }
    pub fn gabr(&self) -> (&T, &T, &T, &T) {
        self.ywzx()
    }

    pub fn bgra(&self) -> (&T, &T, &T, &T) {
        self.zyxw()
    }
    pub fn abgr(&self) -> (&T, &T, &T, &T) {
        self.wzyx()
    }
    pub fn rabg(&self) -> (&T, &T, &T, &T) {
        self.xwzy()
    }
    pub fn grab(&self) -> (&T, &T, &T, &T) {
        self.yxwz()
    }

    pub fn agbr(&self) -> (&T, &T, &T, &T) {
        self.wyzx()
    }
    pub fn ragb(&self) -> (&T, &T, &T, &T) {
        self.xwyz()
    }
    pub fn brag(&self) -> (&T, &T, &T, &T) {
        self.zxwy()
    }
    pub fn gbra(&self) -> (&T, &T, &T, &T) {
        self.yzxw()
    }
}

impl<T: TBytes + Clone> Clone for vec4<T> {
    fn clone(&self) -> Self {
        Self(
            self.0.clone(),
            self.1.clone(),
            self.2.clone(),
            self.3.clone(),
        )
    }
}

impl<T: TBytes + std::fmt::Debug> std::fmt::Debug for vec4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("vec4")
            .field("x", &self.0)
            .field("y", &self.1)
            .field("z", &self.2)
            .field("w", &self.3)
            .finish()
    }
}

impl<T: Add<Output = T>> Add for vec4<T> {
    type Output = vec4<T>;

    fn add(self, rhs: Self) -> Self::Output {
        vec4(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl<T: Sub<Output = T>> Sub for vec4<T> {
    type Output = vec4<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        vec4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl<T: Mul<Output = T>> Mul for vec4<T> {
    type Output = vec4<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        vec4(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
            self.3 * rhs.3,
        )
    }
}

impl<T: Div<Output = T>> Div for vec4<T> {
    type Output = vec4<T>;

    fn div(self, rhs: Self) -> Self::Output {
        vec4(
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2,
            self.3 / rhs.3,
        )
    }
}

impl<T: AddAssign> AddAssign for vec4<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self.3 += rhs.3;
    }
}

impl<T: SubAssign> SubAssign for vec4<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
        self.3 -= rhs.3;
    }
}

impl<T: MulAssign> MulAssign for vec4<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
        self.3 *= rhs.3;
    }
}

impl<T: DivAssign> DivAssign for vec4<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
        self.3 /= rhs.3;
    }
}
