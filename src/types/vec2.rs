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

impl<T: Clone + TBytes> From<(&mut T, &mut T)> for vec2<T> {
    fn from(value: (&mut T, &mut T)) -> Self {
        Self::new(value.0.clone(), value.1.clone())
    }
}

impl<T: TBytes> vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    pub fn x(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn y(&mut self) -> &mut T {
        &mut self.1
    }

    pub fn xy(&mut self) -> (&mut T, &mut T) {
        (&mut self.0, &mut self.1)
    }

    pub fn yx(&mut self) -> (&mut T, &mut T) {
        (&mut self.1, &mut self.0)
    }

    pub fn r(&mut self) -> &mut T {
        self.x()
    }

    pub fn g(&mut self) -> &mut T {
        self.y()
    }

    pub fn rg(&mut self) -> (&mut T, &mut T) {
        self.xy()
    }

    pub fn gr(&mut self) -> (&mut T, &mut T) {
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
