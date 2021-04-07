+++
title = "Binary Tree"
id = "binary_tree"
author = "iain maitland"
description = "A binary tree in Rust"
date = 2021-02-27
tag = "rust"
image = "/assets/og-image-512x512.png"
icon = "🌱"
draft = false
+++

# Binary Tree.
### Creating a Binary Tree

This [gist](https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370) offers a useful starting point when attempting to write a Binary Tree data structure from scratch in Rust.

But doesn't go far to explain why you might want to do this... [Rust provides a far more complete and likely performant `BTreeMap` and `BTreeSet` as part of the `std` library, along with a bunch of useful context for when to use it](https://doc.rust-lang.org/std/collections/index.html). 

The obvious answer is why not, Binary Trees are a fundamental data structure and getting to grips with their internals is a useful learning exercise.
Implementing a binary tree from scratch is useful if you want to implement novel traversals or implement custom methods on your struct. As a recursive data structure, it is also a pretty good introduction to Rust's smart pointers.

### First pass

```
#[derive(Debug)]
struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

impl <T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}


fn main() {
    let my_tree = TreeNode::new(32);
    println!("{:?}", my_tree);
}
```

#### The Box Smart Pointer.
At this point perhaps the most exotic thing about our implementation is the use of the `Box<T>` smart pointer. In Rust a smart pointer lets us give a variable a value of undetermined (at compile time) size. As we would expect, the value is stored on the heap, while the pointer to it, the variable is stored on the stack.

#### But why do we say that the size of our value, is not known at compile time?
In this case the contents of the `Box` is a left or right branch of the Binary Tree, which itself contains more Binary Trees, possibly for ever. For this reason the size of the leaves of the node are not known at compile time. 

> **Takeaway:** In this case the `Box` smart pointer allows us to have a recursive data type.

> **Note:** The Box smart pointer has one weakness, while it can be used for recursive data structures it cannot be used for cyclic ones...[In fact, in Rust, you cannot have cyclical ownership, or at least not easily, this is ok for our purposes as Binary Trees, typically cannot be cyclical.](http://featherweightmusings.blogspot.com/2015/04/graphs-in-rust.html). 

> **Note on a Note:** Cycles can be handled by using pointers other than `Box`, notably `Rc`, `arenas` or the graph data structures in the `petgraph` library, handling cycles might also be a reason for wanting a garbage collector!: 
>> ["Firstly, sometimes you need to manage memory with cycles and Rc<T> is inadequate for the job since Rc-cycles get leaked. petgraph or an arena are often acceptable solutions for this kind of pattern..."](https://manishearth.github.io/blog/2021/04/05/a-tour-of-safe-tracing-gc-designs-in-rust/#why-write-gcs-for-rust)


## Binary Tree Insertion.

Right now we can create a new binary tree, let's go a step further and see how adding nodes to it works out.

```
#[derive(Debug)]
struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

impl <T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
    pub fn insert(&mut self, val: T) {
        self.left = Some(Box::new(TreeNode::new(val)));
    }
}


fn main() {
    let mut my_tree = TreeNode::new(32);
    println!("Initial Tree: {:?}", my_tree);
    my_tree.insert(15);
    println!("After Insert: {:?}", my_tree);
}
```

Here we simply insert a `TreeNode` on the left hand branch of our Binary Tree. This is a bit limited, so let's consider how it can be improved.
1. If the Tree already has the value, do not insert it.
2.  Only insert if the `TreeNode` has an unoccupied branch - i.e. `None` as the branch we want to move into.
3. If the desired branch has a node, move down into it, continue until an unoccupied desired branch is found.

      - When deciding which branch is desired, go left if the Node's value is more than the value to be inserted, and go right if not.
      - Bonus: This will result in a Binary Search Tree!

```
pub fn insert(&mut self, v: T) {
    if self.val == v {
        return;
    }
    
    let target_node = if v < self.val {&mut self.left} else {&mut self.right};
    
    match target_node {
        None => {
            let new_node = Box::new(TreeNode::new(v));
            target_node = Some(new_node);
        },
        Some(node) => {
            node.insert(v)
        }
    }
}
```
There are two important details here, that are not immediately obvious:
1. the variable `target_node` gets a mutable reference to the left or right node. Meaning we can mutate it directly with `target_node = new_node`, at least that's what we'd expect, however, trying to compile the above results in the following error:
```
error[E0308]: mismatched types
  --> src/main.rs:27:31
   |
27 |                 target_node = Some(new_node);
   |                               ^^^^^^^^^^^^^^ expected mutable reference, found enum `Option`
   |
   = note: expected mutable reference `&mut Option<Box<TreeNode<T>>>`
                           found enum `Option<Box<TreeNode<T>>>`
help: consider dereferencing here to assign to the mutable borrowed piece of memory
   |
27 |                 *target_node = Some(new_node);
   |                 ^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
```

Now this leaves us in a bit of a confusing situation, if we have a mutable reference, why can't we mutate it directly?

Here it's crucial to understand that `target_node` is a pointer to an address in memory, not the value that is located there. If we were to mutate it directly, we'd be mutating a pointer, not the data it points to.

Instead we want to mutate the data it is pointing to, and to do that we have to 'follow it', which is what we do by dereferencing, having dereferenced our pointer, we are now acting on the actual data...

> A reference is an address pointer. If you were to just do 
`m += 10`
you'd be changing the memory address (Rust doesn't let you do this without unsafe). What you want to do is change the value at `m`. So where's the value? Follow the pointer! You do this by dereferencing.

Usefully the compiler suggests the following:
```
> help: consider dereferencing here to assign to the mutable borrowed piece of memory
   |
27 |                 *target_node = Some(new_node);
   |                 ^^^^^^^^^^^^
```


Now, to make it clear what our intentions are a  more long winded way of doing this would be:
```
match target_node {
    None => {
        if v < self.val {
            self.left = Some(Box::new(TreeNode::new(v)));
        } else {
            self.right = Some(Box::new(TreeNode::new(v)));
        }
    },
    Some(node) => {
        node.insert(v)
    }
}
```

While using the compiler's suggestion to dereference our mutable reference to mutate the data it refers to, we can do the same thing like this:

```
let target_node = if v < self.val {&mut self.left} else {&mut self.right};

match target_node {
    None => {
        let new_node = Box::new(TreeNode::new(v));
        *target_node = Some(new_node);
    },
    Some(node) => {
        node.insert(v)
    }
}
```

It is worth noting that it is common to want to change the value that a mutable reference refers to, and to use the `dereferencing operator` to do this.

For example:

```
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```
Adds `50` to each element in `v`, in place. 

## Binary Tree Traversal
```
pub fn traverse(&self) {
    println!("{:?}", self.val);
    if let Some(l) = &self.left {
        l.traverse();
    }
    if let Some(r) = &self.right {
        r.traverse();
    }
}
```
This traversal prints the node value followed by the left and right nodes. This is known as `pre-order` traversal.

There are two other types of traversal: `in-order` traversal and `post-order` traversal.

- *Preorder Traversal* - `Current Node, Left Node, Right`
- *Inorder Traversal* - `Left, Current, Right`
- *Postorder Traversal* - `Left, Right, Current`

We can match for the traversal order that we want like, leaving us with this `Binary Tree` implementation:

```
#[derive(Clone, Copy)]
enum Order {
    Pre,
    In,
    Post
}

#[derive(Debug)]
struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

impl <T: std::cmp::PartialEq + std::cmp::PartialOrd + std::fmt::Debug> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
    pub fn insert(&mut self, v: T) {
        if self.val == v {
            return;
        }
        let target_node = if v < self.val {&mut self.left} else {&mut self.right};
        match target_node {
            None => {
                let new_node = Box::new(TreeNode::new(v));
                *target_node = Some(new_node);
            },
            Some(node) => {
                node.insert(v)
            }
        }
    }
    pub fn traverse(&self, order: Order) {
            
        if let Order::Pre = order {
            println!("{:?}", self.val);
        }
        if let Some(l) = &self.left {
            l.traverse(order);
        }
        if let Order::In = order {
            println!("{:?}", self.val);
        }
        if let Some(r) = &self.right {
            r.traverse(order);
        }
        if let Order::Post = order {
            println!("{:?}", self.val);
        }
    }
}


fn main() {
    let mut my_tree = TreeNode::new(32);
    println!("Initial Tree: {:?}", my_tree);
    my_tree.insert(15);
    my_tree.insert(16);
    my_tree.insert(17);
    my_tree.insert(14);
    my_tree.insert(9);
    my_tree.insert(13);
    my_tree.insert(18);
    println!("After Insert: {:?}", my_tree);
    my_tree.traverse(Order::Pre);
    my_tree.traverse(Order::In);
    my_tree.traverse(Order::Post);
}
```

Which altogether isn't too bad! In a follow up I will look at the time and space complexity of different operations on our Tree as well as exploring enhancing it, in particular by referring to the [`std::collections::BTreeMap` implementation in the core library.](https://doc.rust-lang.org/src/alloc/collections/btree/map.rs.html#133-136)
