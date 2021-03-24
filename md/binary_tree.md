+++
title = "Binary Tree"
id = "binary_tree"
author = "iain maitland"
description = "A binary tree in Rust"
date = 2021-02-27
tag = "rust"
image = "/assets/og-image-512x512.png"
icon = "⛓️"
draft = false
+++

# Binary Tree.
### Creating a Binary Tree

As I am still new to rust I often start by writing out the code without thinking about ownership, and then using the compiler to drive further development.Surprisingly, the following is very close to my first pass at implementing a Binary Tree. This was in a large part possible after having read this [gist](https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370).

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
2.  Only insert if the `TreeNode` has an unoccupied branch - i.e. `None` as the left or right branch.
3. If both branches have nodes, move down to one of the branches, continue until an unoccupied branch is found.
3i. When deciding which branch to move into, go left if the Node's value is more than the value to be inserted, and go right if not.

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

This is not compiling because of mismatched types, `target_node` expects a mutable reference to some data in memory, and instead it's getting a chunk of data in memory. We could make `Some(new_node)` a reference, but that's not what our `TreeNode` expects the value of it's `left` or `right` branches to be.


Usefully the compiler provides some help:
```
> help: consider dereferencing here to assign to the mutable borrowed piece of memory
   |
27 |                 *target_node = Some(new_node);
   |                 ^^^^^^^^^^^^
```

What it is suggesting, correctly is that we should assign to the "mutable borrowed piece of memory" that the variable `target_node` points to. To do this we dereference the `target_node` variable, which could either be, `&mut self.left` or `&mut self.right`, and in doing so, convert it into a mutable piece of memory, rather than a mere reference.

A more long winded way of doing this would be:
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

But let's use the compiler's suggestion:

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
This traversal prints the node value followed by the left and right nodes. This is known as pre-order traversal.

There are two other types of traversal are in-order traversal and post-order traversal.

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
    println!("After Insert: {:?}", my_tree);
    my_tree.traverse(Order::Pre);
    my_tree.traverse(Order::In);
    my_tree.traverse(Order::Post);
}
```

Which altogether isn't too bad! In a follow up I will look at the time and space complexity of different operations on our Tree as well as exploring enhancing it, for example creating a balanced binary tree.
