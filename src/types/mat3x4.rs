use crate::prelude::*;
use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct mat3x4<T>(pub vec3<T>, pub vec3<T>, pub vec3<T>, pub vec3<T>);

impl<T: TBytes + Clone> Clone for mat3x4<T> {
    fn clone(&self) -> Self {
        Self(
            self.0.clone(),
            self.1.clone(),
            self.2.clone(),
            self.3.clone(),
        )
    }
}

impl<T: TBytes + Copy> Copy for mat3x4<T> {}
