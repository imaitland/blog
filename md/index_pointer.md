+++
title = "Index Pointer"
id = "index_pointer"
author = "iain maitland"
description = "Index Pointers"
date = 2021-03-07
tag = "rust"
image = "/assets/og-image-512x512.png"
icon = "👈"
draft = false
+++

# Index Pointers

In my post on Linked Lists I allude to this excellent post, [Learn Rust with Entirely Too Many Linked Lists.](https://rust-unofficial.github.io/too-many-lists/)

In that post the author lists many of the reasons why implementing a Linked List from scratch might be a bad idea, and the official rust documentation for `std::collections::LinkedList` nudges us in a similar way... 

> ["It is almost always better to use Vec or VecDeque because array-based containers are generally faster, more memory efficient, and make better use of CPU cache."](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)

But what might they mean by this?!

Are they suggesting that we use a `Vec` instead of a linked list? Or Use a `Vec` *as* as linked list?

I actually think they mean both..., use a vanilla `Vec` whereever possible and where that is insufficient, implement a Linked List by doing something like this:

> The high-level idea is that we will represent a “pointer” to the next item in the list using an index.

```
let my_linked_list = vec![(Some(1), "hello"), (Some(2), "world")];

let mut head = my_linked_list[0];

loop {
    let (next, val) = head;
    match next {
        Some(nx) => {
            println!("{}", val);
            head = my_linked_list[nx];
        },
        _ => {
            break;
        }
    }
}
```

I guess you could call these `index pointers`.