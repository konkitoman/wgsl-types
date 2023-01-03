use crate::prelude::*;
use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct mat4x4<T>(pub vec4<T>, pub vec4<T>, pub vec4<T>, pub vec4<T>);
