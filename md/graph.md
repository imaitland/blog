+++
title = "Graph"
id = "graph"
author = "iain maitland"
description = "A graph in Rust"
date = 2021-02-27
tag = "rust"
image = "/assets/og-image-512x512.png"
icon = "📉"
draft = false
+++
# A tale of two lists

Graphs crop up alot in Computer Science, and life for that matter. Part of their ubiquity is derived from their simplicity. At their core a graph consists of a list of things and a second list of relationships between those things.

In the language of graphs, we call the things in the first list, "Nodes", or "Vertices", and the relationships between them "Edges".

Now, as intimidating as this might seem, its good not to let the fancy language get the better of you, you might of heard of "Breadth First Search" and been a little intimidated, it's the kind of thing brought up by people trying to prove something, which isn't such a bad thing, but if you don't have an academic background, it's enough to put you off... that's a shame, I promise they're useful and just remember, all we're talking about is a list of things, and a list of relationships between those things. This is an incredibly useful way to simplify sometimes complex concepts.

- We might use a list of people, and a list of how those people are related, to represent a family tree.
- We might use a list of towns, and a list of roads between those towns, to find a route from one place to another.
- We might have a list of different positions a chess piece has been in in the course of a game, and another list describing the order of these positions.
- We might use a list of ingredients, and a list of instructions to combine them, to make a delicious meal.
- We might use a list of blog posts, and a list of links between them to represent a blog!

The list goes on. 

Now having accepted their usefulness, and gotten over their general smell of being the stuff of high-fallutin FAANG interviews let's try and implement them.



Now nn Rusty terms, a graph consists of a vector of nodes and a vector of edges and this is reflected by the mathematical definition of a graph:
```
G = (V, E);
```

```
struct Graph {
    head: Option<Box<GraphNode>>
}

struct GraphNode <T> {
    value: T,
    children: Option<Vec<Box<GraphNode>>>
}

impl <T> Graph<T> {
    fn new() -> Self{
        Graph {
            head: None
        }
    }
}