+++
title = "Ownership"
id = "ownership_in_rust"
author = "iain maitland"
description = "Understanding Ownership in Rust"
date = 2021-02-27
tag = "rust"
image = "/assets/og-image-512x512.png"
icon = "🚕"
+++

# Ownership in Rust.
## Introduction

To some extent you can offload the work of holding a mental model of the flow of ownership to the compiler, when it barks at you, you'll snap to attention, start slanging those `to_owned()`, `take()`,`clone()`, `*` and `String::from()` .

However, it's reasonable to assume, having a coherent and correct model of ownership in your software as you write it will keep it idiomatic! After all, ownership is Rust's flagship feature!

Towards this end, here's where I will be putting my notes for all things related to my understanding of ownership. Heavily influenced by the relevant chapter from the rust book.

## Ownership for you.
Ownership consists of 3 things.

1. The Scope
2. The Variable.
3. The Value.

```
fn main() {
  let x = "hello"
}
```
The function body of `main`, bounded by opening and closing `{}` is the **scope**, `x` is the **variable**, and the  `"hello"` is the **value** of type `str`.

These things are related in the following ways, from the bottom up:

The **Value** is stored on the Stack or the heap, depending on its type, values of fixed size, like string slices are stored on the stack, while values of variable size, like Strings are stored on the heap.

The **Variable** and the value are closely intertwined, governed by the following three rules:

1. Each value has one owner, a variable.
2. There can only be one owner variable for each value at a time.
3. A variable and its value are only valid while they are in scope.

> **There's an apparent exception to rule 2**: data on the stack can be cheaply and quickly copied: 
> ```
> let x: str = "hello";
> let y = x;
> ```
> May look like the value "hello" on the stack now has two owners, `x`, and `y`, in fact, the value "hello" a `str` because it exists on the stack can be easily copied, i.e. it implements the `copy()` trait. 
> ```
> let x: String = String::from("hello");
> let y = x;
> ```
> Will not compile due to rule 2. Finally, for completeness - If we did want to copy a value on the heap, and give ownership of the copied data to another variable we can use the `copy` method.
> ```
> let x: String = String::from("hello);
> let y = x.clone();
> 

To summarize thus far - a variable is the owner of a value, and therefore 'owner'  of the space allocated to the value in memory (either the stack or the heap). 

Looking ahead -  the variable is also related to the scope... a variable is only valid while it is in scope, and therefore the space it 'owns' in memory is freed / de-allocated when it moves out of scope.

The **Scope** is the context in which a given variable is valid, typically the code
bounded by a function body. The scope itself is stored on the stack, which has the implication that there is one valid scope at a time (the stack being a Last In First Out queue), with the variables valid within a scope being destroyed when the scope terminates. For example when a function returns, or when there is no more code for a function to run. 

It's worth noting that at this point - the point at which the current scope ends - the scope immediately under it in the stack becomes the valid scope - for example the function body that called the now finished function.

The following summarizes the topics covered so far:
```
fn main () {
  let s = String::from("hello");
  
  takes_ownership(s); 
  // s is now owned by variable for first parameter
  // of takes_ownership.
  
  // using s here wouldn't work!
  
  let p = "hello";
  // p comes into scope.
  
  // p is copied, therefore remains valid.
  let result_2 = fictional_function(p);
  
  // using p here would still work!
  
} 

fn takes_ownership(x: String) -> () {
  // Value previously owned by s is now owned by x within this scope.
  println!("{}", x);
}
// When takes_ownership completes drop() is called and memory occupied
by the value owned by x is freed.
```
  
Altogether, this model makes how values occupy space in memory explicit at compile time, meaning the work of garbage collecting, or autonomously allocating and de allocating memory is unnecessary. Altogether making for fast and memory safe compiled software!

## Borrowing & References

Now - remember we said that there can be one owner of a value at a time, well just like in life, within software we encounter situations where it is convenient to lend and borrow owned entities.

For example one scope might like to pass a value down into another scope, without giving up ownership in the current scope. At this point we can introduce variations on ownership.  We know that a **variable** owns a **value**, but a variable can also (1) be attached to a reference to a value, i.e. a variable can *refer* to a value without owning it. Furthermore, with this reference, a, provided it is *mutable* a (3) variable can have a mutable reference to a value, and with this powerful form of borrowing - can change the value to which it refers!

So ownership can be divided into three types:
1. values
2. references
3. mutable references

Which can be demonstrated like this:

### Values
```
fn main() {
  // The variable x owns the value of type String "hello"
  let x = String::from("hello");
}
```
### References 
```
fn main() {
  let x = String::from("hello");
  does_stuff(&x);
  // Because we passed a reference to does_stuff, it remains in scope, unchanged.
  println!("x: {}", x);
  
}
// This function accepts a reference to a value and gives ownership of that reference to the function parameter y.
fn does_stuff(y: &String) {
  println!("y: {}", y);
}
```
### Mutable References
```
fn main() {

  let x = String::from("hello");
  
  does_stuff(&mut x);
  
  // Because we passed a reference to does_stuff, it remains in scope.
  // Because we passed a mutable reference, it's plausible that x
  // now has a different value, although it remains the owner
  // throughout...
  
  // Prints the mutated value of x, "foo".
  
  println!("x: {}", x);
  
}
// This function accepts a reference to a value and gives
// ownership of that reference to the function parameter y.

fn does_stuff(y: &mut String) {
  String::from("foo");
}
```

The existence of mutable references is governed by three rules.

1. You cannot mutate an immutable reference.
2. There cannot be more than one mutable reference at a time.
3. A mutable reference to a value cannot exist simultaneously with an immutable reference to that value, either all valid references to a value are immutable or immutable.

These three rules are demonstrated by these three compilation errors...

```
fn main() {
  // You cannot mutate an immutable reference.
  // [ ERROR ] "cannot assign twice to immutable variable `y`"
  let x = String::from("hello");
  let y = &x;
  y = &String::from("world");
  
  println!("{}, {}", x, y);

  // There cannot be more than one mutable reference for the same value in scope.
  // [ ERROR ] "cannot borrow `q` as mutable more than once at a time"
  let mut q = String::from("foo");
  let p = &mut q; 
  let f = &mut q;

  println!("{}, {}", p, f);

  // You cannot have both a reference and a mutable reference
  // [ ERROR ] "cannot borrow `q` as mutable because it is also borrowed as immutable"
  let a = String::from("foo");
  let b = &q; 
  let c = &mut q;

  println!("{}, {}", a, b);
}
```
