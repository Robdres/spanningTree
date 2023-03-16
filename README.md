# Implementation of Algorithm to find the minimal spanning tree of a graph

## Running 

		cargo build && cargo run
	
## Description

### Node Structure:

id -> name of node
path -> path to root
neighbors -> all id's of adyecent nodes 

### Graph Structure:

nodes -> HashMap of nodes

## Constructor functions  

- Node::new(i) -> creates a node with id _i_
- Node::new_node(i,p) -> creates a node with id _i_ and path p
- Graph::new() -> creates a empty graph


## Useful functions

- Graph::add_node(), adds a _Node_ in _Graph_
- Graph::add_edges(), connects to _Nodes_ in _Graph_

## MSPA function 

- reduce(g:Graph), takes a _Graph_ and returns another _Graph_ with minimal connections 

