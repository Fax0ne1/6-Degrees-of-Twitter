//crates used
use std::cmp::Ordering;
use std::collections::BinaryHeap;

//defining the priority state for the priority queue
#[derive(Copy, Clone, PartialEq, Eq)] //calling the macro for helper functions to run on the structure
struct state{
    cost: usize,
    position: usize,
    //right now: reach position x with cost y
}

// reverse ordering for a min heap based on the cost/weight (currently in graph set to all 1)
// by defualt it is a max heap, which is the opposite of what is needed
impl Ord for State{
    //implement order trait for the sturcture. Requires ord enum to return greater, less, or equal
    fn cmp(&self, other: &Self) -> Ordering{
        //&self is the current value, comparing it to other. It returns an Ordering value describing the relationship.
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
        //first part, compare to other cost. Then_with() is a method on Ordering that says:
        //If the current comparison result is Equal, run this closure to break the tie.
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

    //keep track of the previous node to reconstruct the path on top of the distance
    let mut prev = vec![None; n];

    //create the priority queue
    let mut heap = BinaryHeap::new();

    //define the start distance to be zero and push to the heap
    dist[start] = 0;
    heap.push(State {cost: 0, position: start });

    //while loop to keep going until the final node is found O((V+E)logV)
    while let Some(State {cost, position}) = heap.pop(){
        //skip greater costs than current
        if cost > dist[position]{continue;}

        //optimize the function, stop when distance is found, don't test everything
        if Some(position) == maybe_target {break;}
        
        //update cost and postion if a smaller one is found. Only visit real edges
        for &(next, weight) in &graph[position]{
            let next_cost = cost + weight;
            if next_cost < dist[next]{
                dist[next] = next_cost;
                prev[next] = Some(position);
                heap.push(State {cost: next_cost, postion: next});
            }
        }
    }
    //return distance
    (dist,prev)
}

//function to rebuild the path
fn build_path(start: usize, dest: usize, prev: &[Option<usize>]) -> Vec<usize>{

}

fn main(){
    //example usage, keeping for refrence, change to work with the actual graph.
    //let dist = dijkstra_adj_list(&graph, start, Some(target));
}