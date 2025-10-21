/* 
    Kalen Hazlett, Group Member 2, Group Member 3
    Version 1.0.0
    Date: 10/21/25
    Purpose: To create a graph as a matrix to store data from reading in a python script and create
             the shortest traversal path
    
    Resources: The code is ours, but I had help from online articles and ChatGPT. I don't have much experience 
               with Rust, and there are articles out there for general form of graphs as data structures. ChatGPT
               helped with compilation errors

*/

//Library to use to help graph
use std::collections::VecDeque;

pub type NodeId = usize;

//graph structure
#[derive(Debug)]
pub struct Graph<N, W = ()>{
    nodes: Vec<N>,
    adj: Vec<Vec<(NodeId,W)>> //creating the matrix (neigbor, weight)
}

//methods to work on the structure above. Like Classes in Python or objects in C++
impl<N, W> Graph<N, W>{
    //public function: creates a new node
    pub fn new() -> Self{
        Self { nodes: Vec::new(), adj: Vec::new()} //create a new node using the vector library imported at top
    }

    //function to add a new node to the graph (&mut means a parameter is going to be a mutable variable)
    pub fn add_node(&mut self, data: N) -> NodeId{
        let id = self.nodes.len(); //creates a variable of usize (size_t)

        self.nodes.push(data);
        self.adj.push(Vec::new());

        //returns the id
        id
    }

    //function to add an edge (u,v) with w (weight)
    pub fn add_edge(&mut self, u: NodeId, v: NodeId, w: W){
        self.adj[u].push((v,w)) //at index u add the values of v and w
    }

    pub fn neighbors(&self, u: NodeId) -> &[(NodeId, W)]{
        &self.adj[u]
    }

    pub fn node(&self, id: NodeId) -> &N { &self.nodes[id] }
    pub fn node_mut(&mut self, id: NodeId) -> &mut N { &mut self.nodes[id]}
    pub fn len(&self) -> usize { self.nodes.len() }
    pub fn is_empty(&self) -> bool { self.nodes.is_empty() }

}

//still works on the structure above, W needs clone
impl<N, W: Clone> Graph<N, W> {
    //function to add an undirected edge (can access v and u without order, ie u <--> v)
    pub fn add_undir_edge(&mut self, u: NodeId, v: NodeId, w: W){
        self.add_edge(u, v, w.clone());
        self.add_edge(v, u, w);
    }
}

//unweighted BFS, ie, put all weights as 1
    pub fn bfs<N, W>(g: &Graph<N, W>, start: NodeId) -> Vec<Option<NodeId>>{

        let mut parent = vec![None; g.len()]; //option is like an enum, ie, parent may/may not exist
        let mut q = VecDeque::new();
        
        parent[start] = Some(start); // some is a an option constructor. Basically, avoid null pointers.
                                     // ie, Some means that something has been found in the option
        q.push_back(start);

        while let Some(u) = q.pop_front(){
            for &(v, _) in g.neighbors(u){
                if parent[v].is_none() {
                    parent[v] = Some(u);
                    q.push_back(v);
                }
            }
        }
        parent
    }

// ------- demo -------
fn main() {
    let mut g = Graph::<&'static str, f32>::new();
    let a = g.add_node("A");
    let b = g.add_node("B");
    let c = g.add_node("C");
    let d = g.add_node("D");

    g.add_undir_edge(a, b, 1.0);
    g.add_undir_edge(b, c, 2.5);
    g.add_undir_edge(a, d, 0.7);

    // Traverse
    let parent = bfs(&g, a);

    
    // Reconstruct a path A -> C
    let mut path = Vec::new();
    let mut cur = c;
    loop {
        path.push(cur);
        if cur == a { break; }       
        cur = parent[cur].unwrap(); // safe if C is reachable from A
    }
    path.reverse();

    println!("Path A->C (by ids): {:?}", path);
    println!("Path labels: {:?}", path.iter().map(|&id| g.node(id)).collect::<Vec<_>>());
    
}