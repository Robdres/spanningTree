#![allow(dead_code,non_snake_case)]
use std::collections::{HashSet,HashMap};
use queues::*;

#[derive(Debug, Clone)]
struct Node {
    id: i32,
    path:Vec<i32>,
    neighbors: HashSet<i32>,
}

impl Node {
    fn new(id: i32) -> Self {
        Node {
            id,
            path:vec![],
            neighbors: HashSet::new(),
        }
    }

    fn new_node(id: i32,path:Vec<i32>) -> Self {
        Node {
            id,
            path:path,
            neighbors: HashSet::new(),
        }
    }

    fn change_path(&mut self, path:Vec<i32>){
        self.path = path;
    }

    fn add_neighbor(&mut self, neighbor_id: i32) {
        self.neighbors.insert(neighbor_id);
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
    nodes: HashMap<i32, Node>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn get_node(&self ,node_id:i32) -> Node {
        let n:Node;
        if let Some(node) = self.nodes.get(&node_id){
            n = node.to_owned()
        }else {
            n = Node::new(0)
        }
        n
    }

    // fn change_node_path(&mut self, node_id:i32, path:Vec<i32>){
    //     let aux = self.nodes.get
    //     self.nodes.
    // } 

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

        if let Some(node) = self.nodes.get_mut(&neighbor_id) {
            node.add_neighbor(node_id);
        } 
    }

}

//method to reduce graph
fn reduce(root_id: i32, mut graph:Graph) -> Graph { 
    let mut visited = HashMap::new();
    visited.insert(0, 0);

    let mut queue:Queue<i32>= Queue::new();

    let root = graph.get_node(root_id);
    let mut mst = Graph::new();
    mst.add_node(root);

    for i in graph.get_node(root_id).neighbors.into_iter(){
        mst.add_node(Node::new_node(i,vec![0,i]));
        graph.nodes.get_mut(&i).expect("hello").change_path(vec![0,i]);
        mst.add_edge(root_id, i);
        visited.insert(i, 1);
        queue.add(i).expect("Only one Root");
    }

    while queue.peek().is_ok(){
        let node_id:i32 = queue.remove().expect("No next neighboors");
        let node = graph.nodes.get(&node_id).expect("error Node").clone();
        for i in node.neighbors.iter(){
            let mut new_path =  node.path.clone();
            new_path.push(*i);
            if visited.get(i).is_some() {
                if *visited.get(i).expect("hello") < new_path.len() as i32{
                    visited.remove(i);
                    visited.insert(*i,new_path.len() as i32);
                }
            }else{
                eprintln!("node_id = {:?}", node_id);
                eprintln!("new_path = {:?}", new_path);
                queue.add(*i).expect("nothing");
                visited.insert(*i, new_path.len() as i32);
                let aux = new_path.clone();
                mst.add_node(Node::new_node(*i, new_path));
                graph.nodes.get_mut(i).expect("hello").change_path(aux);
                mst.add_edge(node_id, *i);
            }
        }
    }
    
    mst
}
fn add_to_path(path:&HashSet<i32>,value:i32) -> Vec<i32>{
    let mut new_path:Vec<i32> = vec![];
    for i in path.iter(){
        new_path.push(*i);
    }
    new_path.push(value);
    new_path
}

fn main() {
    let b0= Node::new(0);
    let b1= Node::new(1);
    let b2= Node::new(2);
    let b3= Node::new(3);
    let b4= Node::new(4);
    let b5= Node::new(5);
    let b6= Node::new(6);
    let b7= Node::new(7);

    let mut test_graph = Graph::new();

    test_graph.add_node(b0);
    test_graph.add_node(b1);
    test_graph.add_node(b2);
    test_graph.add_node(b3);
    test_graph.add_node(b4);
    test_graph.add_node(b5);
    test_graph.add_node(b6);
    test_graph.add_node(b7);
    test_graph.add_edge(0, 1);
    test_graph.add_edge(1, 2);
    test_graph.add_edge(0, 2);
    test_graph.add_edge(2, 3);
    test_graph.add_edge(2, 4);
    test_graph.add_edge(3, 4);
    test_graph.add_edge(2, 5);
    test_graph.add_edge(5, 6);
    test_graph.add_edge(6, 7);

    eprintln!("test_graph = {:?}", test_graph);
    let mst = reduce(0, test_graph);
    eprintln!("mst = {:?}", mst);

}
