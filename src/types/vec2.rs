use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct vec2<T> {
    pub x: T,
    pub y: T,
}

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

impl<T: Clone + TBytes> From<(&mut T, &mut T)> for vec2<T> {
    fn from(value: (&mut T, &mut T)) -> Self {
        Self::new(value.0.clone(), value.1.clone())
    }
}

impl<T: TBytes> vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&mut self) -> &mut T {
        &mut self.x
    }

    pub fn y(&mut self) -> &mut T {
        &mut self.y
    }

    pub fn xy(&mut self) -> (&mut T, &mut T) {
        (&mut self.x, &mut self.y)
    }

    pub fn yx(&mut self) -> (&mut T, &mut T) {
        (&mut self.y, &mut self.x)
    }

    pub fn r(&mut self) -> &mut T {
        &mut self.x
    }

    pub fn g(&mut self) -> &mut T {
        &mut self.y
    }

    pub fn rg(&mut self) -> (&mut T, &mut T) {
        (&mut self.x, &mut self.y)
    }

    pub fn gr(&mut self) -> (&mut T, &mut T) {
        (&mut self.y, &mut self.x)
    }
}

impl<T: TBytes + PartialEq> PartialEq for vec2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: TBytes + Clone> Clone for vec2<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T: TBytes + std::fmt::Debug> std::fmt::Debug for vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("vec2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
