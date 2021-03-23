+++
title = "Remainder"
id = "remainder"
author = "iain maitland"
description = "Working with remainder in Rust"
date = 2021-02-27
tag = "rust"
image = "/assets/og-image-512x512.png"
icon = "➗"
draft = false
+++

# Understanding `%` and `rem_euclid()` in Rust.

For two positive numbers, `%` and `rem_euclid` methods behave the same way, both will give you the remainder for the division of one number by another. Where they are different is how they treat negative numbers. Or Rather, where some confusion arrises is in deciding what convention to adopt. In some contexts having a negative remainder makes sense, in others it does not. Generally, negative remainders are not really useful.

- In Rust `%` Remainder will always return a remainder matching the sign of the left hand side of the operation:
```
println!("{}", 6 % -4); // expect 2.
println!("{}", -10 % 7); // expect -3.
```
This means we might have a negative remainder. In cases where this is not desired, how do we find the remainder?

`rem_euclid()` provides an alternative and is generally preffered.
```
let a: i32 = -10;
let b: i32 = 7;

let c: i32 = -20;
let d: i32 = 8;

println!("{}", a % b); // -3
println!("{}", a.rem_euclid(b)); // 4

println!("{}", c.rem_euclid(d)); // 4
```
#### So what's happening here?!
Note the different results, 
`a % b` is `-3` because it's simply calculated the remainder. 

`a.rem_euclid(b)` is `4` because we take the first quotient of b that overshoots `-10`, and then add a number to walk back to our target, in this case: `7 * -2 = -14`, `-14 + 4 = 10`, so our 'remainder' is 4.

Similarly, the remainder from dividing -20 by 8, is `8 * -3 = -24` which we walk back to `-20` by adding `4`, so our, 'remainder' is 4.