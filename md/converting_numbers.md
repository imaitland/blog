+++
title = "Numbers"
id = "converting_numbers"
author = "iain maitland"
description = "Converting numbers in Rust"
date = 2021-03-11
tag = "rust"
image = "/assets/og-image-512x512.png"
icon = "🧮"
draft = true
+++

# Converting numbers

```rust
use std::convert::TryFrom;

fn main() {
    let a: f64 = 9.99999;
    let b: i32 = 1;
    let z: isize = 999999;
    
    // Best way to get from a lower resolution number to a higher resolution one.
    // Can be either float or integer.
    let v = b as f64;
    
    // Best way to get from a higher resolution integer to a lower resolution one.
    // Can only be integer -> integer, or float -> float.
    let c = i32::try_from(z).unwrap() + b;
    
    // Only way to accurately get from a float to an integer.
    // Can be float -> integer.
    fn convert(x: f64) -> i32 {
        x.round().rem_euclid(2f64.powi(32)) as u32 as i32
    }
    
}
```