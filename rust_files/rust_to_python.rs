// Kalen Hazlett, Morgan Q, Phillip S.
// Purpose: File for the communication between rust and python, second attempt, different way
// Version: 2.5.0
    // Code is ours, though ChatGPT was used to help generate ideas for how to do this and debug. 
    // PyO3 seemed too complex of a way in the time we had. (A way to use python functions in rust) 
    // We decided to settle on stdin and out to read to/from the programs
    // this is just communicating between two individual running scripts. 
// Resources:
    // https://users.rust-lang.org/t/getting-started-with-rust-and-json/128674
    // https://thelinuxcode.com/rust-json-example/ 
    // https://blog.logrocket.com/json-and-rust-why-serde_json-is-the-top-choice/

//crates used used
use crate::graph::{Graph, NodeId};
//use std::collections::VecDeque;
use serde_json;
use crate::inputs::get_input;
use serde::Serialize;
use std::collections::{HashMap, HashSet, VecDeque}; //can take out hashset if take out last function
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::fs::File;
use std::io;
//don't need to keep these two in, but they are there for .env file reading, code block commented out though
use std::env;
use dotenvy::dotenv;

//the structure to hold the information coming from python json. 
//May be a small bug with username, don't have time to fix, and it works...
#[derive(serde::Deserialize)]
struct PyResp {
    username: String,
    mutuals: Option<Vec<String>>,
    error: Option<String>,
}

/* 
fn normalize_all(v: impl IntoIterator<Item=String>) -> Vec<String> {
    v.into_iter().map(|s| canon(&s)).collect()
}*/

//unweighted BFS, ie, all weights as 1. 
fn bfs_until<N, W>(g: &Graph<N, W>, start: NodeId, target: NodeId) -> Vec<Option<NodeId>> {
    let mut parent = vec![None; g.len()];
    let mut q = VecDeque::new();
    parent[start] = Some(start);// 'some' is a an option constructor. Basically, avoid null pointers if it doesn't exist.
                                // ie, Some means that something has been found in the option
    q.push_back(start);

    //loop through the list O(v+e)
    while let Some(u) = q.pop_front() {
        if u == target { break; } //exit early if the end is found
        for &(v, _) in g.neighbors(u) {
            if parent[v].is_none() {
                parent[v] = Some(u);
                q.push_back(v);
            }
        }
    }
    parent //return parent
}

//function to recreate the path for, later, visualization
fn build_path_from_parent(start: NodeId, dest: NodeId, parent: &[Option<NodeId>]) -> Vec<NodeId> {
    //create a new vector for the path
    let mut path = Vec::new();
    let mut cur = Some(dest);
    //while there are still nodes, loop to recontruct the path
    while let Some(v) = cur {
        path.push(v);
        if v == start { break; } //break when no more.
        cur = parent[v];
    }
    path.reverse();
    if path.first().copied() == Some(start) { path } else { Vec::new() } //check for start
}

// 'Book keeping' helpers
fn canon(s: &str) -> String { s.trim().to_ascii_uppercase() } //normalizing input function. *Should* reduce bad inputs
//function to create a node or get the id of a node
fn get_or_create_node(name: &str, g: &mut Graph<String, f64>, idx: &mut HashMap<String, NodeId>) -> NodeId {
    let key = canon(name);//noramlize as above
    if let Some(&id) = idx.get(&key) {
        id //return the created id
    } else {
        let id = g.add_node(key.clone());// store canonical label in the graph
        idx.insert(key, id);
        id //return
    }
}
//boolean function to check if a graph has edges 
fn has_edge(g: &Graph<String, f64>, u: NodeId, v: NodeId) -> bool {
    g.neighbors(u).iter().any(|p| p.0 == v)
}

// For saving BFS result and outputting it to jSON
#[derive(Serialize)]
struct BfsResult {
    start_user: String,
    end_user: String,
    path_nodes: Vec<String>,
    path_ids: Vec<usize>,
    distance: usize,
}


//where the 'magic' happens. Starts the communication with pyhthn and sends data
pub fn pipeline() -> std::io::Result<()> {

    /* 
    //setting up an .env to read in path names. didn't want to hardcode them 
    dotenv().ok();
    // Get your variable
    let data_path = env::var("DATA_PATH")
        .expect("DATA_PATH not found in .env or environment");

    println!("Reading from: {}", data_path); */

    //spawn the python script as a sub process
    let mut child = Command::new("python3")
        .arg("-u")
        .arg("python_script.py")//change for actual script name. Use .env file if needed/wanted
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    //creating variables for data
    let mut py_stdin = child.stdin.take().expect("failed to open python stdin");
    let py_stdout = child.stdout.take().expect("failed to open python stdout");
    let mut reader = BufReader::new(py_stdout);

    //create some variables for traversing later
    let mut g: Graph<String, f64> = Graph::new();
    let mut index: HashMap<String, NodeId> = HashMap::new();

    //could have read in from a file or gotten input, but why? this works for the sample data we have
    let usernames = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", 
                                    "k", "l", "m", "n", "o", "p", "q", "r",
                                    "s", "t", "u", "v", "w", "x", "y", "z"];
    //loop through the list of usernames, parsing each one
    for u in &usernames {
        //normalize the input
        let username = canon(u);
        // send username to the python script
        writeln!(py_stdin, "{}", username)?;
        py_stdin.flush()?; //flush the buffer

        let mut line = String::new();
        let n = reader.read_line(&mut line)?; //read the python line
        //basic error checking
        if n == 0 {
            eprintln!("Python exited unexpectedly");
            break;
        }

        // Ensure the node for this username exists
        let u_id = get_or_create_node(&username, &mut g, &mut index);

        // a mtach is like a c switch. find the one it matches and execute for parsing
        match serde_json::from_str::<PyResp>(&line) {//serde is json crate
            Ok(resp) => {
                if resp.error.is_some() {
                    // user not found: keep the node, add no edges
                } else if let Some(mutuals) = resp.mutuals { //look for the mutuals
                    // normalize and add undirected edges for each mutual
                    for m in mutuals.into_iter().map(|s| canon(&s)) {
                        if m == username { continue; }// no self-loop
                        //create a node or add an edge in both directions. Could have created a undir edge, ran into some errors
                        let v_id = get_or_create_node(&m, &mut g, &mut index);
                        if !has_edge(&g, u_id, v_id) { g.add_edge(u_id, v_id, 1.0); }
                        if !has_edge(&g, v_id, u_id) { g.add_edge(v_id, u_id, 1.0); }
                    }
                } else {
                    // json parsed but had no mutuals key
                    eprintln!("No mutuals field for {}: {}", username, line.trim());
                }
            }
            Err(_) => {
                eprintln!("Non-JSON or unexpected response for {}: {}", username, line.trim());
            }
        }
    }

    // close the connection
    drop(py_stdin);
    let _ = child.wait();
    //debug checks 
    let edge_count: usize = (0..g.len()).map(|u| g.neighbors(u).len()).sum();
    println!("Graph has {} nodes and {} edges.", g.len(), edge_count);
    println!("Index has {} keys. Has A? {}  Has a? {}",
            index.len(),
            index.contains_key("A"),
            index.contains_key("a"));

    //Inspect graph safely without requiring a copy of w
    println!("Graph has {} nodes.", g.len());
    //loop throught the graph and print the results
    for u in 0..g.len() {
        println!("node[{u}] = {:?}", g.node(u));
        for &(v, ref w) in g.neighbors(u) {
            println!("  --> {v} (w={:?})", w);
        }
    }

    // Get start and end names. Get input is defined in the inputs file.
    let start_node = get_input("Enter start node:").trim().to_ascii_uppercase();
    let end_node   = get_input("Enter end node:").trim().to_ascii_uppercase();

    // Map to ids and handle missing
    let (start, dest) = match (index.get(&start_node), index.get(&end_node)) {
        (Some(&s), Some(&d)) => (s, d),
        _ => {
            //eror
            eprintln!("Start or end username not found in index.");
            return Ok(());
        }
    };

    // BFS shortest path (fewest hops) and save results as json
    //could try dikjtra's if this doesn't work as intended
    let parent = bfs_until(&g, start, dest);
    let path_ids = build_path_from_parent(start, dest, &parent);
    //something went wrong if this shows or data doesn't support the path (mostly used for debugging though)
    if path_ids.is_empty() {
        println!("No path from {start_node} to {end_node}");
    //store data in the structure to send to json
    } else {
        let path_names: Vec<String> = path_ids.iter().map(|&u| g.node(u).clone()).collect();
        println!("Shortest path ({} steps): {:?}", path_ids.len().saturating_sub(1), path_names);

        // Save BFS result as a json so it can be read into anbother pthon script
        let bfs_result = BfsResult {
            start_user: start_node.clone(),
            end_user: end_node.clone(),
            path_nodes: path_names,
            distance: path_ids.len().saturating_sub(1),
            path_ids,
        };
        //actual json majic output happens here
        serde_json::to_writer_pretty(File::create("bfs_result.json")?, &bfs_result)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }

    // Save graph
    g.write_json_pretty("graph.json", /*directed=*/false)?;

    Ok(())//return ok, because using '?' for error checking
}


/* 
// helper: parse one accounts.tx style line and compute mutuals for the expected user
//no longer used because it isn't plain text
fn mutuals_from_accounts_line(line: &str, expected: &str) -> Option<Vec<String>> {
    let s = line.trim();
    if s.is_empty() { return None; }

    let (user, rest) = s.split_once(':')?;
    if canon(user) != canon(expected) { return None; }

    let (left, right) = rest.split_once('|').unwrap_or((rest, ""));

    let to_set = |side: &str| -> HashSet<String> {
        side.split(',')
            .map(|t| canon(t))   // <<< normalize each neighbor name
            .filter(|t| !t.is_empty())
            .collect()
    };

    let followers = to_set(left);
    let following = to_set(right);

    let mutuals: Vec<String> = followers.intersection(&following).cloned().collect();
    Some(mutuals)
}
*/