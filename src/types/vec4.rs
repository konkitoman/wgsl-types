use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct vec4<T>(pub T, pub T, pub T, pub T);

impl<T: Clone + TBytes> From<T> for vec4<T> {
    fn from(value: T) -> Self {
        Self::new(value.clone(), value.clone(), value.clone(), value)
    }
}

impl<T: TBytes> From<(T, T, T, T)> for vec4<T> {
    fn from(value: (T, T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl<T: Clone + TBytes> From<(&mut T, &mut T, &mut T, &mut T)> for vec4<T> {
    fn from(value: (&mut T, &mut T, &mut T, &mut T)) -> Self {
        Self::new(
            value.0.clone(),
            value.1.clone(),
            value.2.clone(),
            value.3.clone(),
        )
    }
}

impl<T: TBytes> vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self(x, y, z, w)
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

    pub fn w(&mut self) -> &mut T {
        &mut self.3
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

    pub fn xw(&mut self) -> (&mut T, &mut T) {
        (&mut self.0, &mut self.3)
    }
    pub fn wx(&mut self) -> (&mut T, &mut T) {
        (&mut self.3, &mut self.0)
    }

    pub fn zw(&mut self) -> (&mut T, &mut T) {
        (&mut self.2, &mut self.3)
    }
    pub fn wz(&mut self) -> (&mut T, &mut T) {
        (&mut self.3, &mut self.2)
    }

    pub fn xyw(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.0, &mut self.1, &mut self.3)
    }
    pub fn wxy(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.3, &mut self.0, &mut self.1)
    }
    pub fn ywx(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.1, &mut self.3, &mut self.0)
    }

    pub fn yxw(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.1, &mut self.0, &mut self.3)
    }
    pub fn wyx(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.3, &mut self.1, &mut self.0)
    }
    pub fn xwy(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.0, &mut self.3, &mut self.1)
    }

    pub fn xwz(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.0, &mut self.3, &mut self.2)
    }
    pub fn zxw(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.2, &mut self.0, &mut self.3)
    }
    pub fn wzx(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.3, &mut self.2, &mut self.0)
    }

    pub fn wxz(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.3, &mut self.0, &mut self.2)
    }
    pub fn zwx(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.2, &mut self.3, &mut self.0)
    }
    pub fn xzw(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.0, &mut self.2, &mut self.3)
    }

    pub fn xyzw(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3)
    }
    pub fn wxyz(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.3, &mut self.0, &mut self.1, &mut self.2)
    }
    pub fn zwxy(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.2, &mut self.3, &mut self.0, &mut self.1)
    }
    pub fn yzwx(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.1, &mut self.2, &mut self.3, &mut self.0)
    }

    pub fn yxzw(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.1, &mut self.0, &mut self.2, &mut self.3)
    }
    pub fn wyxz(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.3, &mut self.1, &mut self.0, &mut self.2)
    }
    pub fn zwyx(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.2, &mut self.3, &mut self.1, &mut self.0)
    }
    pub fn xzwy(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.0, &mut self.2, &mut self.3, &mut self.1)
    }

    pub fn xywz(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.0, &mut self.1, &mut self.3, &mut self.2)
    }
    pub fn zxyw(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.2, &mut self.0, &mut self.1, &mut self.3)
    }
    pub fn wzxy(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.3, &mut self.2, &mut self.0, &mut self.1)
    }
    pub fn ywzx(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.1, &mut self.3, &mut self.2, &mut self.0)
    }

    pub fn xzyw(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.0, &mut self.2, &mut self.1, &mut self.3)
    }
    pub fn wxzy(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.3, &mut self.0, &mut self.2, &mut self.1)
    }
    pub fn ywxz(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.1, &mut self.3, &mut self.0, &mut self.2)
    }
    pub fn zywx(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.2, &mut self.1, &mut self.3, &mut self.0)
    }

    pub fn wyzx(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.3, &mut self.1, &mut self.2, &mut self.0)
    }
    pub fn xwyz(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.0, &mut self.3, &mut self.1, &mut self.2)
    }
    pub fn zxwy(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.2, &mut self.0, &mut self.3, &mut self.1)
    }
    pub fn yzxw(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.1, &mut self.2, &mut self.0, &mut self.3)
    }

    pub fn xwzy(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.0, &mut self.3, &mut self.2, &mut self.1)
    }
    pub fn yxwz(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.1, &mut self.0, &mut self.3, &mut self.2)
    }
    pub fn zyxw(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.2, &mut self.1, &mut self.0, &mut self.3)
    }
    pub fn wzyx(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        (&mut self.3, &mut self.2, &mut self.1, &mut self.0)
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
    pub fn a(&mut self) -> &mut T {
        self.w()
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

    pub fn rga(&mut self) -> (&mut T, &mut T, &mut T) {
        self.xyw()
    }
    pub fn arg(&mut self) -> (&mut T, &mut T, &mut T) {
        self.wxy()
    }
    pub fn gar(&mut self) -> (&mut T, &mut T, &mut T) {
        self.ywx()
    }

    pub fn gra(&mut self) -> (&mut T, &mut T, &mut T) {
        self.yxw()
    }
    pub fn agr(&mut self) -> (&mut T, &mut T, &mut T) {
        self.wyx()
    }
    pub fn rag(&mut self) -> (&mut T, &mut T, &mut T) {
        self.xwy()
    }

    pub fn rab(&mut self) -> (&mut T, &mut T, &mut T) {
        self.xwz()
    }
    pub fn bra(&mut self) -> (&mut T, &mut T, &mut T) {
        self.zxw()
    }
    pub fn abr(&mut self) -> (&mut T, &mut T, &mut T) {
        self.wzx()
    }

    pub fn rgba(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.xyzw()
    }
    pub fn argb(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.wxyz()
    }
    pub fn barg(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.zwxy()
    }
    pub fn gbar(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.yzwx()
    }

    pub fn grba(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.yxzw()
    }
    pub fn agrb(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.wyxz()
    }
    pub fn bagr(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.zwyx()
    }
    pub fn rbag(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.xzwy()
    }

    pub fn rgab(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.xywz()
    }
    pub fn brga(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.zxyw()
    }
    pub fn abrg(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.wzxy()
    }
    pub fn gabr(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.ywzx()
    }

    pub fn bgra(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.zyxw()
    }
    pub fn abgr(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.wzyx()
    }
    pub fn rabg(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.xwzy()
    }
    pub fn grab(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.yxwz()
    }

    pub fn agbr(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.wyzx()
    }
    pub fn ragb(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.xwyz()
    }
    pub fn brag(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        self.zxwy()
    }
    pub fn gbra(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
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
