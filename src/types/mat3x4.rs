use crate::prelude::*;
use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct mat3x4<T>(pub vec3<T>, pub vec3<T>, pub vec3<T>, pub vec3<T>);
