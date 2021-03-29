+++
title = "Ownership"
id = "ownership_in_rust"
author = "iain maitland"
description = "Understanding Ownership in Rust"
date = 2021-02-27
tag = "rust"
image = "/assets/og-image-512x512.png"
icon = "🚕"
draft = false
+++

# Ownership in Rust.
## Compiler driven development.

It can be seductive to offload the work of keeping a mental model of the flow of ownership in your Rust program to the compiler. I wouldn't want to discourage this approach - it's a good way to develop an intuitive understanding of ownership as well as one way to be productive in Rust quickly.

However, you eventually run into a situation where the compiler and its helpful warnings fall short, and you find yourself having to carefully follow the thread ownership has woven through your software. As you find yourself doing this more often, it becomes clear that having a coherent and correct model of ownership in your software as you write it, is unavoidable. 

This isn't such a bad thing... after all, ownership is Rust's flagship feature with many advantages. At the very least understanding how ownership works will help your code remain idiomatic and make compiler errors easier to fix.

Towards this end, here's where I will be putting my notes for all things related to my understanding of ownership. Heavily influenced by the relevant chapter from the rust book.

## Ownership for you.
Ownership consists of 3 things.

1. The Scope
2. The Variable.
3. The Value.

So that this function defintion:
```
fn main() {
  let x = "hello"
}
```
Can be understood like this:

- The function body of `main`, bounded by opening and closing curly-brackets `{}` is the **scope**
- `x` is the **variable**
- `"hello"` is the **value** of type `str`.

> 🐬 All three of these things exist in the computer's memory, which itself is divided into two parts the stack and the heap. Discussing these in great depth is beyond the scope of this post, suffice it to say: A **value** can be stored in the stack or the heap depending on its type, while **variables** and **scope** exist on the stack.

The **variable** and the **value** are closely related, with their relationship governed by the following three rules:

1. Each value has one owner, a variable.
2. There can only be one variable for each value at a time.
3. A variable and its value are only valid while they are in scope.

> ➿ **There's an apparent exception to rule 2**: Data on the stack can be cheaply and quickly copied: 
> ```
> let x: str = "hello";
> let y = x;
> ```
> May look like the value "hello" on the stack now has two owners, `x`, and `y`.
>
> However, when we assigned the value of variable `x` to `y` we cloned the value.
>
> This is because the value in question - "hello" is an `str`, and because it exists on the stack can be easily copied, i.e. it implements the `copy()` trait.
> 
> Trying to do the same thing with a `String` (which exists on the heap)...
> ```
> let x: String = String::from("hello");
> let y = x;
> ```
> Does not compile due to rule 2.
> 
> Finally, for completeness - If we did want to copy a value on the heap, and give ownership of the copied data to another variable we can use the `clone` method.
> ```
> let x: String = String::from("hello);
> let y = x.clone();
> ```
> 
>

To summarize thus far - a **variable** is the **owner** of a value, and therefore 'owner'  of the space allocated to the value in memory (either on the stack or in the heap). 

The **variable** is also related to the **scope** - remember rule 3.
> "A **variable** and its **value** is only valid while it is in scope"

By implication the space a **variable** 'owns' in memory is freed / de-allocated when its scope ends.

Which begs the question, when does the **scope** begin and end?

The **scope** is the context in which a given variable is valid, typically the code bounded by a function body. So it can be said that the scope begins with the opening curly-bracket `{` and ends with the closing `}` curly bracket.

This understanding neatly plays out in practice, especially when we have nested functions... it is clear that there is only one valid scope at a time.

> 🔭 The scope itself is stored on the stack, which has the implication that there is one valid scope at a time (the stack being a Last In First Out queue), with the variables valid within a scope being destroyed when the scope terminates. For example when a function returns, or when there is no more code for a function to run. 

## You are the garbage collector 🚮

So we've seen how **values** in the stack or heap are owned by **variables** and remain accessible as long as they are in **scope**.

Each of these things has an explicit lifecycle that is defined by the boundaries of the scope, they do not persist in memory beyond these bounds and because the compiler enforces this - the work of freeing memory when it is no longer in use is done by us as we write our software! 

You might hear people say that Rust doesn't have a garbage collector, and it is this that they are referring to, simply put it does not need one because memory management is explicitly encoded in the software. It's a bit like we are conscripted as the garbage collector that rust is missing. 

Whether or not you agree with that - the following summarizes the topics covered so far:
```
fn main () {
  let s = String::from("hello");
  
  takes_ownership(s); 
  // s is now owned by variable for first parameter
  // of takes_ownership.
  
  // using s here wouldn't work!
  
  let p = "hello";
  // p comes into scope.
  
  // p is copied, remember it is a str! and therefore remains valid.
  let result_2 = fictional_function(p);
  
  // using p here would still work!
  
} 

fn takes_ownership(x: String) -> () {
  // Value previously owned by s is now owned by x
  // within this scope.
  println!("{}", x);
}
// When takes_ownership completes drop() is called and memory occupied
by the value owned by x is freed.
```

## Ownership for you and for fast and safe software.
Altogether, this model makes how values occupy space in memory explicit at compile-time. This means the work of garbage collecting, or autonomously allocating and deallocating memory, characteristic of other languages is unnecessary in Rust. Altogether making for fast and memory-safe compiled software!

## Borrowing & References

In life, we are familiar with borrowing and its rules. Say we rent a car we understand this entitles us to use it but not change it. If we were to change the vehicle, say by swapping its engine, we'd be in trouble, because the original owner would not know what they had anymore! Likewise, if the rental company decided to scrap the car during our rental period we'd be a little annoyed, as something we expected to exist no longer does. 

![](/assets/2020.6.29-Silverado-Swap.jpg)
> 🛻 Things get interesting when an immutable reference is treated as a mutable one.

While swapping out a UHaul engine is against the rules of car rentals, we do also have situations where we want to be able to change the thing that we borrow, for example when you take a stained dress to the dry cleaner or a car to a mechanic. In these cases - we grant permission to the borrower, to make changes to the thing that they have borrowed.

Well just like in life, within software we encounter situations where it is convenient to lend and borrow owned entities, with rules governing whether or not the borrowed thing can be changed or not.

In the context of Rust, we frequently encounter these situations. For example, one scope might like to pass a value down into the scope of another function, without giving up ownership in the current scope, and without having to `clone()` the value. At this point, we can introduce variations on ownership.
 - A **variable** can own a **value**. (We're familiar with this)
 - A **variable** can also own a reference to a value, i.e. a variable can *refer* to a value without owning it.
 - If this *reference* is mutable then the value to which it refers can be changed.

So the ownership a variable can have of something in memory can be divided into three types:

1. values
2. references
3. mutable references

In the analogous context of real life, you can own a car that you can lend to others, you can rent a car that is immutable, and you can be a mechanic that borrows and mutates/changes cars.

### Examples

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
  // Because we passed an immutable reference to does_stuff, it remains in scope, unchanged.
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
3. A mutable reference to a value cannot exist simultaneously with an immutable reference to that value, either all valid references to a value are immutable or there is one mutable reference.

These three rules are demonstrated by these three compilation errors...

```
fn main() {
  // You cannot mutate an immutable reference.
  let x = String::from("hello");
  let y = &x;
  y = &String::from("world");
  
  println!("{}, {}", x, y);
  // [ ERROR ] "cannot assign twice to immutable variable `y`"

  // There cannot be more than one mutable reference for the same value in scope.
  let mut q = String::from("foo");
  let p = &mut q; 
  let f = &mut q;

  println!("{}, {}", p, f);
  // [ ERROR ] "cannot borrow `q` as mutable more than once at a time"

  // You cannot have both a reference and a mutable reference
  let a = String::from("foo");
  let b = &q; 
  let c = &mut q;

  println!("{}, {}", a, b);
  // [ ERROR ] "cannot borrow `q` as mutable because it is also borrowed as immutable"
}
```
### Borrowing the family car

Using powerful things typically comes with rules, and ownership with its rules for borrowing and references is the framework that helps us write systems-level code in a responsible and safe way. It crops up all the time and is therefore really important to get a firm handle of, something that takes a while. Fortunately the compiler will be there to hold our hand as our understanding and intuition grows.

For the time being, lets recap the rules

#### Rules of ownership. 
1. Each value has one owner, a variable.
2. There can only be one variable for each value at a time.
3. A variable and its value are only valid while they are in scope.

#### Rules of references.
1. You cannot mutate a value using an immutable reference to it.
2. There cannot be more than one mutable reference to a value at a time.
3. A mutable reference to a value cannot exist simultaneously with an immutable reference to that value, either all valid references to a value are immutable or there is one mutable reference at a time.

And it's gotta be returned with a full tank of gas!



