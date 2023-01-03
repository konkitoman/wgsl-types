use crate::prelude::*;
use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct mat3x2<T>(pub vec3<T>, pub vec4<T>);

impl<T: TBytes + Clone> Clone for mat3x2<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<T: TBytes + Copy> Copy for mat3x2<T> {}
