use crate::prelude::*;
use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct mat2x3<T>(pub vec2<T>, pub vec3<T>, pub vec2<T>);
