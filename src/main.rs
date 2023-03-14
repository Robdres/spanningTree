use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    id: i32,
    neighbors: Vec<i32>,
}

impl Node {
    fn new(id: i32) -> Self {
        Node {
            id,
            neighbors: vec![],
        }
    }

    fn add_neighbor(&mut self, neighbor_id: i32) {
        self.neighbors.push(neighbor_id);
    }

    fn add_neighbors(&mut self, ids: Vec<i32>) {
        for i in ids.iter(){
            self.add_neighbor(i as *const i32 as i32);
        }
    }

    fn remove_neighbor(&mut self, neighbor_id: i32) {
        self.neighbors.retain(|x| *x != neighbor_id);
    }
}

#[derive(Debug)]

struct Graph {
    root: i32,
    nodes: HashMap<i32, Node>,
}

impl Graph {

    fn new() -> Self {
        Graph {
            root: 0,
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    fn add_edge(&mut self, node_id: i32, neighbor_id: i32) {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.add_neighbor(neighbor_id);
        } else {
            let mut node = Node::new(node_id);
            node.add_neighbor(neighbor_id);
            self.nodes.insert(node_id, node);
        }
    }
}


fn reduce(root: i32, mut graph:Graph) -> Graph{ 
    let ans = Graph::new();
}

fn main() {
    let b1= Node::new(1);
    let testGraph = Graph::new();
    testGraph.add_node(b1);


    let b2= Node::new(2);
    let b3= Node::new(3);
    let b4= Node::new(4);
    let b5= Node::new(5);
    let b6= Node::new(6);
    let b7= Node::new(7);
    let b8= Node::new(8);
    println!("Hello, world!");
}
