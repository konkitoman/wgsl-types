use crate::prelude::*;
use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct mat4x2<T>(pub vec4<T>, pub vec4<T>);

impl<T: TBytes + Clone> Clone for mat4x2<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<T: TBytes + Copy> Copy for mat4x2<T> {}
