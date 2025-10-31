//crates used
use std::cmp::Ordering;
use std::collections::BinaryHeap;

//defining the priority state for the priority queue
#[derive(Copy, Clone, ParialEq, Eq)] //calling the macro for helper functions to run on the structure
struct state{
    cost: uszie,
    postion: usize,
    //right now: reach position x with cost y
}

// reverse ordering for a min heap based on the cost/weight (currently in graph set to all 1)
// by defualt it is a max heap, which is the opposite of what is needed
impl Ord for State{
    fn cmp(&self, other: &self){
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//function for the dijktra traversal.
//Arguments: graph double vector defined by the structure (the graph) the start node, target node
//return vector (list) of usize
fn dkj_adj_list(graph: &Vec<Vec<(usize,usize)>>, start: usize, maybe_target: Option<usize>) -> Vec<usize>{
    let n = graph.len(); //create a variable for the graph length.

    //define the final distance to be 'infinity' (really big number)
    let mut dist = vec![usize::MAX; n];
    
    //create the priority queue
    let mut heap = BinaryHeap::new();

    //define the start distance to be zero and push to the heap
    dist[start] = 0;
    heap.push(State {cost: 0, postion: start });

    //while loop to keep going until the final node is found ~O(n^2)
    while let Some(State {cost, position}) = heap.pop(){
        //skip greater costs than current
        if cost > dist[position]{continue;}
        
        //update cost and postion if a smaller one is found. Only visit real edges
        for &(next, weight) in &graph[position]{
            let next_cost = cost + weight;
            if next_cost < dist[next]{
                dist[next] = next_cost;
                heap.push(State {cost: next_cost, postion: next});
            }
        }
    }
    //return distance
    dist
}

fn main(){
    //example usage, keeping for refrence, change to work with the actual graph.
    //let dist = dijkstra_adj_list(&graph, start, Some(target));
}