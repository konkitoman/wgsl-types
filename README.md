# WGSL-TYPES

[![Crates.io](https://img.shields.io/crates/v/wgsl-types.svg)](https://crates.io/crates/wgsl-types)

## supports

- vec2

- vec3

- vec4

is uses bytes-kman for serialization and deserialization

## Example

```rust
use bytes_kman::prelude::*;
use wgsl_types::prelude::*;


fn main(){
    let a = vec3::<f32>(1.0, 1.0, 0.0);
    
    // this is the bytes!
    let mut bytes = a.to_bytes();
    // if you want to create a vec3 from bytes you need to have the bytes in reverse
    bytes.reverse();
    // the bytes will be consumed
    let b = <vec3::<f32>>::from_bytes(&mut bytes).unwrap();
}
```
