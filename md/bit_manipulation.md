+++
title = "Bit Manipulation"
id = "bit_manipulation"
author = "iain maitland"
description = "Working with binary in Rust"
date = 2021-02-27
tag = "rust"
image = "/assets/og-image-512x512.png"
icon = "🏁"
draft = false
+++

# Binary
Binary is a way to write any number as a sum of 2 to the power of n. Where n is the position of the factor of 2 in the binary string.

For example:
- `1 = 2^0 = 1`
- `10 = 2^1 + 0 = 2`
- `100 = 2^2 + 0 + 0 = 4`
- `111 = 2^2 + 2^1 + 2^0 = 4 + 2 + 1 = 7`

It is common to have to convert binary numbers to regular numbers and vice versa.

As an aside, to convert decimal numbers to binary we will be using Rust's `%` remainder operator, not `rem_euclid()` [I wrote a little about their difference.](/remainder)

### Convert regular number to binary.
```
fn convert_to_binary(input: i32) -> Vec<i32>{
    let mut current = input;
    let mut binary = vec![];
    while current > 0 {
        match current % 2 {
            0 => {
                binary.push(0);
                current = current/2;
            },
            1 => {
                binary.push(1);
                current = (current-1)/2;
            }
        }
    } 
    binary
}
```

### Convert a binary number to a regular number.
```
fn convert_to_regular(input: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..input.len() {
        match input[i] {
            0 => {
                result += 0;
            },
            1 => {
                let new_value = 2.pow(input.len() -1 -i ); // 
                result += result + new_value;
            }
        }
    }
    result
}
```

#### Understanding 2's complement.

We must find a way to represent negative numbers in binary.
- For a positive number, represented in binary, flip its bits (1's complement) and add 1 to get its negative representation in binary.
```
0101 = 5;
1010 = -8 + 2 = -6
Remember the +1!
1011 = -8 + 2 + 1 = -5
```
- Note, we can identify negative numbers in binary that can represent them because they begin with 1.