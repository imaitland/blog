+++
title = "Graph"
id = "graph"
author = "iain maitland"
description = "Implementing a graph in Rust"
date = 2021-03-06
tag = "graph"
image = "/assets/og-image-512x512.png"
icon = "🦥"
draft = false
+++

# A tale of two lists

Graphs crop up a lot in life. Part of their usefulness comes from their simplicity. At their core, they consist of two lists. 
1. A list of things
2. A list of links between those things.

- We might use a list of people and a list of how they are related, representing a family tree.
- We might use a list of towns, and a list of roads between those towns, to find a route from one place to another or draw a map.
- We might have a list of different positions a chess piece has been in in the course of a game and another list describing how each position followed the previous one.
- [We might use a list of atoms and bonds between them to represent a molecule.](https://depth-first.com/articles/2020/02/17/graphs-in-rust-introducting-graphcore/)
- We might use a list of blog posts and links between them to represent a blog.

In the language of graphs, we call the things in the first list *nodes* and the connections between them *edges.* 

Edges, nodes and, the graphs they make up can have different properties.

- They can be *sparse* (when nodes don't have many edges each) or *dense* (when nodes have a lot of edges between them).

- When the nodes in a graph are assigned geometric positions, we say the graph is *embedded*; sometimes, these graphs don't need edges at all. In these cases, edges are derived from the nodes' embeddedness, for example, deriving interplanetary gravitational forces purely from the relative distance between bodies and their respective masses or finding the shortest delivery route for a truck, knowing the locations of the stores it must visit.

- When a cycle exists, we say the graph is *cyclic*; if a cycle is impossible or non-useful, then the graph is *acyclic*.  Could you imagine a graph representation of your favorite recipe being cyclic? or a family tree for that matter!?

![](https://www.traditioninaction.org/OrganicSociety/Images_1-100/A_019_Hapsburgs.jpg)

> The European Hapsburg dynasty - perhaps it should've been an acyclic graph?

- When edges in our graph have an additional property beyond the node they come from and the node they go to, we often say they are *weighted*. Weighted edges have a weight property that stores some critical value for the relationship that it represents. Roads between cities have distances which we can include as the 'weight' of our edges.

- When edges in our graph have a specific direction property, we have a *directed* graph. A Graph of friends would be *undirected* since we like to assume the relationship is reciprocal. A graph of intersections and roads would be *directed* because streets are often one-way.

We can see that graphs can have specific properties derived from their constituent nodes and edges. They can be sparse, embedded, cyclic, weighted, directed, or any combination of those things and their obverses!

Now, as intimidating as this might seem, it's good not to let the fancy language get the better of you; more often than not, the type of graph you want will emerge as you work. 

It is enough to remember that any graph can be represented by two lists, in mathematical terms:
```
G = (N, E);
```
In terms of Rust, we can say a graph consists of a vector of nodes and a vector of edges.
```
Graph:
    N0 ---E0---> N1 ---E1---> 2
    |                         ^
    E2                        |
    |                         |
    v                         |
    N3 ----------E3-----------+
```


Let's try and represent this graph:

There are a few ways we could approach this; one involves an `adjacency  matrix`:

```
N  0    1    2    3
0  0    1    0    1
1  0    0    1    0
2  0    0    0    0
3  0    0    1    0
```
See what's happening there?  Edges are represented by a `1` at coordinates `(NX, NY).`

In Rust, our adjacency matrix for this graph could look like this.
```
let N0 = vec![0,1,0,1];
let N1 = vec![0,0,1,0];
let N2 = vec![0,0,0,0];
let N3 = vec![0,0,1,0];

let graph = vec![N0,N1,N2,N3];
```

One characteristic of this approach is that we're using our nodes' indices to identify and access them. When we iterate over the list at `graph[0]`, we can find, by implication, all the edges between it `Node 0` and other nodes. In the case of finding a `1`, at index `1` and `3`, we know that `Node 0` identified by `graph[0]` has edges connecting it to `Node 1` and `Node 3`. that is, for `Node 0` edges `0->1` and `0->3` exist.

```
let mut from_index = 0;

while node_index < graph.len() {
    let mut to_index = 0;
    let mut current_node = graph[from_index];
    while to_index < current_node.len() {
        match has_edge {
            0 => {},
            1 => {
                println!("Edge, from: {from_index}, to: {to_index}");
            }
        }
    }
    from_index += 1;
}
```

Another way of representing this is to treat the edges as our data. To do this, we might define our graph as a list of edges.
```
let edges = [
    (0,1),(0,3),
    (1,2),
    (3,2)
];
```
We could then derive our nodes from our edge data, for example:

```
let mut nodes = vec![];

for edge in edges.iter() {
    let (from, to) = edge;
    for i in 0..std::cmp::max(*from,*to) + 1 {
        if i >= nodes.len() {
            nodes.push(i as i32);
        }
    }
}

println!("Nodes: {:?}", nodes);
```

Reviewing the adjacency matrix approach, particularly this last one, we can note that it has some strengths.

- It makes it easy to find if an edge exists in our graph.
- It makes it easy to find if a node exists in our graph.
- Edge insertion and deletion are rapid.

[This approach is similar to the implementation of a `matrix_graph` in the `petgraph` library.](https://docs.rs/petgraph/0.5.1/petgraph/matrix_graph/struct.MatrixGraph.html#method.from_edges)

However, the adjacency matrix graph does have a flaw - for sparse graphs with many nodes and relatively few edges, it uses excessive space, `O(N*N),` this issue is compounded in the case of undirected graphs where nodes can share edges.

> Imagine a graph of intersections within a gridded American city, with 3,000 intersections, each representing where two roads cross, we have 6,000 edges (remember junctions share edges) we end up with an adjacency matrix of `3,000 * 3,000 = 9,000,000` cells, with most (9,000,000 - 6,000) of them being empty!

At this point, our implementation tends away from an `adjacency matrix` and relies more on `adjacency lists`; these are better at representing sparse graphs; they do this by using a linked list to store the edges adjacent to each node. `O(N+E)`

As we saw in the [Index Pointer](/index_pointer) post, we can represent linked lists as vectors using `index pointers`.

```
pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

pub type NodeIndex = usize;

pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
}
pub type EdgeIndex = usize;

pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>
}
```

> As you can see, an edge contains a target node index and an optional index for the next outgoing edge. All edges in a particular linked list share the same source, which is implicit. Thus there is a linked list of outgoing edges for each node that begins in the node data for the source and is threaded through each of the edge datas.
** source:** ["Modeling graphs in Rust using vector indices"](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/#the-high-level-idea)

Let's have a shot at writing an `add_node` and `add_edge` method for our struct:
```
impl Graph {
    fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }
    fn add_node(&mut self) -> NodeIndex {
        let ix = self.nodes.len();
        self.nodes.push(NodeData{first_outgoing_edge: None});
        ix
    }
    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) -> EdgeIndex {
        let ix = self.edges.len();
        let node_data = &mut self.nodes[from];
        // Don't worry about duplicate edges for now.
        self.edges.push(
            EdgeData {
                target: to,
                next_outgoing_edge: node_data.first_outgoing_edge
            }
        );
        node_data.first_outgoing_edge = Some(ix);
        ix
    }
}
```

Not too shabby, right now, we can build our example graph like this:
```
let mut my_graph = Graph::new();

my_graph.add_node();
my_graph.add_node();
my_graph.add_node();
my_graph.add_node();

my_graph.add_edge(0,1);
my_graph.add_edge(0,3);
my_graph.add_edge(1,2);
my_graph.add_edge(3,2);
```

But where our implementation shines is in traversing our graph. Let's write a method that will allow us to do that:

```
impl Graph {
    fn traverse(&mut self, curr: usize) {
        if self.visited[curr] == true {
            return;
        };
        // Get the first outgoing edge for our node.
        match self.nodes[curr].first_outgoing_edge {
            Some(mut edge_index) => {
                // Traverse the target node for this edge...
                self.traverse(self.edges[edge_index].target);
                // Find all connected nodes and traverse them...
                while let Some(next) = self.edges[edge_index].next_outgoing_edge {
                    // update edge_index.
                    edge_index = next;
                    // traverse the target node for this edge.
                    self.traverse(self.edges[edge_index].target);
                }
            },
            None => {
                // No outgoing edges on this node.
                println!("No connected nodes.");
            }
            
        }
        println!("Visited Node: {}", curr);
        self.visited[curr] = true;
    }
}
```

Importantly! ecause we used an index as a pointer to relate our `nodes` and `edges,` we didn't have to dive into the tricky territory of smart pointers like `Rc`!

> "This approach plays very well to Rust's strengths. This is because, unlike an Rc pointer, an index alone is not enough to mutate the graph: you must use one of the &mut self methods in the graph. This means that can track the mutability of the graph as a whole in the same way that it tracks the mutability of any other data structure."

The next step will be to finesse our `traversal` method using an iterator; for example, wouldn't it be nice to do something like this:

```
let my_node = my_graph.successors(node_index);
let connected_nodes = my_node.iter().collect();
```

Finally, we'll have a shot at implementing a method for detecting cycles!

Review questions:
- Is it `depth-first search` or `breadth-first search` that we have implemented?
- How might we implement the other search algorithm.


Reading list:
- [Graphs in Rust](http://featherweightmusings.blogspot.com/2015/04/graphs-in-rust.html)
- [Modeling Graphs in Rust Using Vector Indices](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/)
- [A Minimal Graph Api](https://depth-first.com/articles/2020/01/06/a-minimal-graph-api/)
- [Graphs in Rust, Introducing Graph Code](https://depth-first.com/articles/2020/02/17/graphs-in-rust-introducting-graphcore/)