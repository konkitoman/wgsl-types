use crate::prelude::*;
use bytes_kman::prelude::*;

#[derive(Bytes)]
pub struct mat4x2<T>(pub vec4<T>, pub vec4<T>);
