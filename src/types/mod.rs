#![allow(non_camel_case_types)]

mod mat2x2;
mod mat2x3;
mod mat2x4;
mod mat3x2;
mod mat3x3;
mod mat3x4;
mod mat4x2;
mod mat4x3;
mod mat4x4;
mod vec2;
mod vec3;
mod vec4;

pub mod prelude {
    pub use super::mat2x2::*;
    pub use super::mat2x3::*;
    pub use super::mat2x4::*;
    pub use super::mat3x2::*;
    pub use super::mat3x3::*;
    pub use super::mat3x4::*;
    pub use super::mat4x2::*;
    pub use super::mat4x3::*;
    pub use super::mat4x4::*;
    pub use super::vec2::vec2;
    pub use super::vec3::vec3;
    pub use super::vec4::vec4;
}
