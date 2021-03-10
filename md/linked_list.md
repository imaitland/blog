+++
title = "Linked List"
id = "linked_list"
author = "iain maitland"
description = "A Linked List in Rust"
date = 2021-02-27
tag = "rust"
image = "/assets/og-image-512x512.png"
icon = "⛓️"
draft = false
+++

# Linked List

## first pass.

This linked list with `new` and `push` implementations demonstrates and important concept regarding Ownership.

```
#![allow(unused)]
fn main() {
    struct List<T> {
        head: Link<T>
    };
    
    type Link<T> = Option<Box<Node<T>>>;
    
    struct Node<T> {
        elem: T,
        next: Link<T>
    }
    
    impl<T> List<T> {
    
        fn new() -> Self {
            List {
                head: None
            }
        }
        
        fn push(&mut self, e: T) {
            let new_node = Box::new(Node {
                elem: e,
                next: self.head
            });
            self.head = Some(new_node);
        }
        
    }
}
```

In the above implementation, there's the following compilation error:
```
31 |                 next: self.head
   |                       ^^^^^^^^^
   |                       |
   |                       move occurs because `self.head` has type `Option<Box<Node<T>>>`, which does not implement the `Copy` trait
   |                       help: consider borrowing the `Option`'s content: `self.head.as_ref()`
```

Understanding the problem:

1. The variable `self` enters the push method as a mutable reference to the List.

2. The mutable reference to a value   has its ownership moved to from the variable `self.head` to the variable `new_node.next`:
  - At this point, `self.head` is a variable without a value...
    - Another way of putting this would be to say that the value has two owners, `self.head` and `new_node.next`.
  - We know that we'll later give `self.head` a valid value - `Some(new_node)`
  - **BUT** the compiler doesn't want to leave this up to chance, what if we forgot, then we risk accessing `self.head` and it pointing to a location in memory that it does not own, or that does not exist! We're violating the rule that a value can only have one owner.
3. Therefore we need a way of passing ownership to `new_node.next` while stuffing a pacifier in `self.head`'s mouth to stop it crying out, at least until we can give it a useful value...

The Solution.

Unfortunately in this case the compiler's suggestion, won't be exactly what we want. That gives `new_node.next` ownership of a *reference*, but in this case, the `next` field of the Node struct cannot be a reference.

Our solution is: [`take`](https://doc.rust-lang.org/std/mem/fn.take.html) in the docs it states:
> ... allows taking ownership of a struct field by replacing it with an "empty" value. Without take you can run into issues...

In our language, we use "empty" as a pacifier for the `self.head` variable, until we can assign it an useful value.

Great! let's use this nifty feature to solve our ownership woes and continue, adding a pop and find method!

```
struct List<T> {
    head: Link<T>
}

type Link<T> = Option<<Box<T>>>

struct Node<T> = {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    new() -> Self {
        List {
            head: None
        }
    }
    push(&mut self, e: T) {
        let new_node = Node {
            elem: e,
            next: self.head.take()
        }
       self.head = new_node
    }
    pop(&mut self) -> Option<T> {
        match self.head.take() { // after calling self.head.take(), self.head gets the 'empty' pacifier.
            None => None,
            Some(n) => {
                self.head = n.next; // here we rehydrate the empty self.head with something meanigful.
                Some(n.elem)
            }
        }
    }
    find(&self, needle: T, curr: Node<T>) -> Option<Node<T>> {
        match self.head {
            None => None,
            Some(n) => {
                if (n.elem) == needle {
                    return n
                }
            }
        }

    }
}
