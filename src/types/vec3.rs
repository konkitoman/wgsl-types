use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Clone + TBytes> From<T> for vec3<T> {
    fn from(value: T) -> Self {
        Self {
            x: value.clone(),
            y: value.clone(),
            z: value,
        }
    }
}

impl<T: TBytes> From<(T, T, T)> for vec3<T> {
    fn from(value: (T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl<T: Clone + TBytes> From<(&mut T, &mut T, &mut T)> for vec3<T> {
    fn from(value: (&mut T, &mut T, &mut T)) -> Self {
        Self::new(value.0.clone(), value.1.clone(), value.2.clone())
    }
}

impl<T: TBytes> vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn x(&mut self) -> &mut T {
        &mut self.x
    }

    pub fn y(&mut self) -> &mut T {
        &mut self.y
    }

    pub fn z(&mut self) -> &mut T {
        &mut self.z
    }

    pub fn xy(&mut self) -> (&mut T, &mut T) {
        (&mut self.x, &mut self.y)
    }
    pub fn yx(&mut self) -> (&mut T, &mut T) {
        (&mut self.y, &mut self.x)
    }

    pub fn xz(&mut self) -> (&mut T, &mut T) {
        (&mut self.x, &mut self.z)
    }
    pub fn zx(&mut self) -> (&mut T, &mut T) {
        (&mut self.z, &mut self.x)
    }

    pub fn yz(&mut self) -> (&mut T, &mut T) {
        (&mut self.y, &mut self.z)
    }
    pub fn zy(&mut self) -> (&mut T, &mut T) {
        (&mut self.z, &mut self.y)
    }

    pub fn xyz(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.x, &mut self.y, &mut self.z)
    }
    pub fn zxy(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.z, &mut self.x, &mut self.y)
    }
    pub fn yzx(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.y, &mut self.z, &mut self.x)
    }

    pub fn yxz(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.y, &mut self.x, &mut self.z)
    }
    pub fn zyx(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.z, &mut self.y, &mut self.x)
    }
    pub fn xzy(&mut self) -> (&mut T, &mut T, &mut T) {
        (&mut self.x, &mut self.z, &mut self.y)
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

impl<T: TBytes + PartialEq> PartialEq for vec3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: TBytes + Clone> Clone for vec3<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}

impl<T: TBytes + std::fmt::Debug> std::fmt::Debug for vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("vec3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}
