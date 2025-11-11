/* 
    Kalen Hazlett, Morgan Q, Phillip S.
    Version 2.2.0
    Date: 10/21/25
    Purpose: To create a graph as a list and store data from reading in a python script and create
             the shortest traversal path using Dijktra's algorithm.
    
    Resources: The code is ours, but I had help from online articles and ChatGPT. I don't have huges amounts of experience 
               with Rust, and there are articles out there for general form of graphs as data structures. ChatGPT
               helped with compilation errors and transalation of languages (C --> Rust && Rust --> C).
            
               https://www.geeksforgeeks.org/dsa/adjacency-matrix/
               https://timclicks.dev/tip/how-to-create-a-2d-matrix-in-rust
               https://www.geeksforgeeks.org/c/c-program-to-implement-adjacency-list/
               https://medium.com/@kriangkrai.ratt/shortest-path-dijkstra-algorithm-with-rust-8fe1867d052a 

*/

//Library to use to help graph
use std::collections::VecDeque;

//type alias for usize (size_t in C)
pub type NodeId = usize;

//graph structure, generatic form so it is compatible with alot of data types
#[derive(Debug)] //macro to make printing format easier.
pub struct Graph<N, W = ()>{
    nodes: Vec<N>, //creates the nodes to 
    adj: Vec<Vec<(NodeId,W)>> //creating the list (neigbor, weight)
}

//methods to work on the structure above. Like Classes in Python or objects in C++
impl<N, W> Graph<N, W>{
    //public function: create a new node
    pub fn new() -> Self{
        //create a new node using the vector library imported at top Vectors are lists that can change size safely
        Self { nodes: Vec::new(), adj: Vec::new()} 
    }

    //function to add a new node to the graph (&mut means a parameter is going to be a mutable variable)
    pub fn add_node(&mut self, data: N) -> NodeId{
        let id = self.nodes.len(); //creates a variable of usize

        self.nodes.push(data);
        self.adj.push(Vec::new());

        //returns the id
        id
    }

    //function to add an edge (u,v) with w (weight)
    pub fn add_edge(&mut self, u: NodeId, v: NodeId, w: W) {
        assert!(u < self.adj.len() && v < self.nodes.len(), "invalid node id"); //assert is a simple 'test' function
        self.adj[u].push((v, w));
    }

    pub fn neighbors(&self, u: NodeId) -> &[(NodeId, W)]{
        &self.adj[u]
    }

    pub fn node(&self, id: NodeId) -> &N { &self.nodes[id] }
    //pub fn node_mut(&mut self, id: NodeId) -> &mut N { &mut self.nodes[id]}
    pub fn len(&self) -> usize { self.nodes.len() }
    //pub fn is_empty(&self) -> bool { self.nodes.is_empty() }

}

//still works on the structure above, W needs clone though (copy itself)
// undirected helper (requires W: Clone to copy the weight)
impl<N, W: Clone> Graph<N, W> {
    pub fn add_undir_edge(&mut self, u: NodeId, v: NodeId, w: W) {
        self.add_edge(u, v, w.clone());
        self.add_edge(v, u, w);
    }
}
//unweighted BFS, ie, put all weights as 1
pub fn bfs<N, W>(g: &Graph<N, W>, start: NodeId, target: NodeId) -> Vec<Option<NodeId>>{

    let mut parent = vec![None; g.len()]; //option is like an enum, ie, parent may/may not exist
    let mut q = VecDeque::new();
    
    parent[start] = Some(start); // 'some' is a an option constructor. Basically, avoid null pointers if it doesn't exist.
                                 // ie, Some means that something has been found in the option
    q.push_back(start);

    //loop through the list O(v+e)
    while let Some(u) = q.pop_front(){
        if u == target {break;}
        for &(v, _) in g.neighbors(u){
            if parent[v].is_none() {
                parent[v] = Some(u);
                q.push_back(v);
            }
        }
    }
    parent //return parent
}

//function to build the shortest path
pub fn build_path_from_parent(start: NodeId,dest: NodeId,parent: &[Option<NodeId>],) -> Vec<NodeId> {
    let mut path = Vec::new();
    let mut cur = Some(dest);

    while let Some(v) = cur {
        path.push(v);
        if v == start { break; }
        cur = parent[v];
    }
    path.reverse();
    if path.first().copied() == Some(start) { path } else { Vec::new() }
}

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct BfsResult {
    start_user: String,
    end_user: String,
    path_nodes: Vec<String>,
    path_ids: Vec<usize>,
    distance: usize,
}