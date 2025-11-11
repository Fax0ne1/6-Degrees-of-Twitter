// File for the communication between rust and python, second attempt, different way
// Code is ours, though ChatGPT was used to help generate ideas for how to do this and debug. PyO3 seemed to complex
// of a way to do this in the time we had. (A way to use python functions in rust) 
// We decided to settle on stdin and out to read to/from the programs
// this is just communicating between to individual running scripts. 

//libraries used
use crate::graph::{Graph, NodeId};
use serde_json;
use crate::inputs::get_input;
use serde::{Serialize,Deserialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::fs::File;
use std::io;

// Python response if we were using a json (not any longer, could probably get rid of it)
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum PyResp {
    Found { username: String, mutuals: Vec<String> },
    NotFound { username: String, error: String },
}

//python response as plain text
enum PlainResp {
    Mutuals(Vec<String>),
    NotFound,
}

// BFS function
use std::collections::VecDeque;
fn bfs_until<N, W>(g: &Graph<N, W>, start: NodeId, target: NodeId) -> Vec<Option<NodeId>> {
    let mut parent = vec![None; g.len()];
    let mut q = VecDeque::new();
    parent[start] = Some(start);
    q.push_back(start);
    while let Some(u) = q.pop_front() {
        if u == target { break; } //exit early if the end is found
        for &(v, _) in g.neighbors(u) {
            if parent[v].is_none() {
                parent[v] = Some(u);
                q.push_back(v);
            }
        }
    }
    parent
}

//function to recreate the path for, later, visualization
fn build_path_from_parent(start: NodeId, dest: NodeId, parent: &[Option<NodeId>]) -> Vec<NodeId> {
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

// 'Bookkeeping' helpers
fn get_or_create_node(name: &str, g: &mut Graph<String, f64>, idx: &mut HashMap<String, NodeId>) -> NodeId {
    if let Some(&id) = idx.get(name) {
        id
    } else {
        let id = g.add_node(name.to_owned());
        idx.insert(name.to_owned(), id);
        id
    }
}

fn has_edge(g: &Graph<String, f64>, u: NodeId, v: NodeId) -> bool {
    g.neighbors(u).iter().any(|p| p.0 == v)
}

// For saving BFS result
#[derive(Serialize)]
struct BfsResult {
    start_user: String,
    end_user: String,
    path_nodes: Vec<String>,
    path_ids: Vec<usize>,
    distance: usize,
}

// Plain-text parser, forgiving about formats (hopefully)
fn parse_plain_response(line: &str) -> PlainResp {
    let s = line.trim();
    if s.is_empty() { return PlainResp::NotFound; }

    let lower = s.to_ascii_lowercase();
    if lower.contains("not found") || lower.contains("error") {
        return PlainResp::NotFound;
    }

    // Accept multiple ways "b,c", "mutuals: b, c", "alice: b,c", "username -> b,c", "b | c | d" (we'll treat '|' like ',' as a convenience)

    //normalize the input
    let rhs = s
        .split_once(':')// "mutuals: b,c" | "alice: b,c"
        .map(|(_, r)| r)
        .or_else(|| s.split_once("->").map(|(_, r)| r))
        .unwrap_or(s);

    let rhs = rhs.replace('|', ","); // normalize the separators
    let mutuals: Vec<String> = rhs
        .split(',')
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect();

    if mutuals.is_empty() {
        PlainResp::NotFound
    } else {
        PlainResp::Mutuals(mutuals)
    }
}

pub fn pipeline() -> std::io::Result<()> {
    //spawn the python script as a sub process
    let mut child = Command::new("python3")
        .arg("-u")
        .arg("script.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut py_stdin = child.stdin.take().expect("failed to open python stdin");
    let py_stdout = child.stdout.take().expect("failed to open python stdout");
    let mut reader = BufReader::new(py_stdout);

    //create some variables for traversing later
    let mut g: Graph<String, f64> = Graph::new();
    let mut index: HashMap<String, NodeId> = HashMap::new();

    //need to change this later
    let usernames = vec!["a", "b", "c"];

    //loop through the list of usernames, parsing each one
    for u in &usernames {
        // send username to the python script
        writeln!(py_stdin, "{}", u)?;
        py_stdin.flush()?;

        // read exactly one response line (plain text or python) couldn't keep track of which one we were using
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        //errror chekcings
        if n == 0 {
            eprintln!("Python exited unexpectedly");
            break;
        }

        // try JSON format first, then plain text
        let mut handled = false;

        //use the json library to check if it works
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(line.trim_end()) {
            if let (Some(username), Some(mutuals_val)) =
                (json_val.get("username").and_then(|v| v.as_str()), json_val.get("mutuals"))
            {
                if let Some(arr) = mutuals_val.as_array() {
                    let mutuals: Vec<String> = arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();

                    let u_id = get_or_create_node(username, &mut g, &mut index);
                    for m in mutuals {
                        if m == username { continue; }
                        let v_id = get_or_create_node(&m, &mut g, &mut index);
                        let w = 1.0;
                        if !has_edge(&g, u_id, v_id) { g.add_edge(u_id, v_id, w); }
                        if !has_edge(&g, v_id, u_id) { g.add_edge(v_id, u_id, w); }
                    }
                    handled = true;
                }
            } else if json_val.get("error").is_some() {
                handled = true; // treat as not found
            }
        }

        //parse the plain text response (probably will be what we end up using)
        if !handled {
            //match is like a switch case in c, matching the parsed response from the function above
            match parse_plain_response(&line) {
                PlainResp::Mutuals(mutuals) => {
                    let username: &str = *u; // because u is &&str
                    //update nodes
                    let u_id = get_or_create_node(username, &mut g, &mut index);
                    //check for mutuals
                    for m in mutuals {
                        if m == username { continue; }
                        let v_id = get_or_create_node(&m, &mut g, &mut index);
                        let w = 1.0;
                        //add in both directions to make it an undirected edge
                        if !has_edge(&g, u_id, v_id) { g.add_edge(u_id, v_id, w); }
                        if !has_edge(&g, v_id, u_id) { g.add_edge(v_id, u_id, w); }
                    }
                }
                //if something is not found, ie, error checking
                PlainResp::NotFound => {
                    let username = (*u).to_string();
                    let _ = get_or_create_node(&username, &mut g, &mut index);
                }
            }
        }
    }

    //write a line to python to stop the process after the loop finishes
    writeln!(py_stdin, "exit")?;
    drop(py_stdin);
    let _ = child.wait();

    // Inspect graph safely without requiring a copy of w
    println!("Graph has {} nodes.", g.len());
    for u in 0..g.len() {
        println!("node[{u}] = {:?}", g.node(u));
        for &(v, ref w) in g.neighbors(u) {
            println!("  --> {v} (w={:?})", w);
        }
    }

    // Get start and end names
    let start_node = get_input("Enter start node:");
    let end_node   = get_input("Enter end node:");

    // Map to ids (handle missing)
    let (start, dest) = match (index.get(&start_node), index.get(&end_node)) {
        (Some(&s), Some(&d)) => (s, d),
        _ => {
            eprintln!("Start or end username not found in index.");
            return Ok(());
        }
    };

    // BFS shortest path (fewest hops) and save results as json
    let parent = bfs_until(&g, start, dest);
    let path_ids = build_path_from_parent(start, dest, &parent);

    if path_ids.is_empty() {
        println!("No path from {start_node} to {end_node}");
    } else {
        let path_names: Vec<String> = path_ids.iter().map(|&u| g.node(u).clone()).collect();
        println!("Shortest path ({} steps): {:?}", path_ids.len().saturating_sub(1), path_names);

        // Save BFS result
        let bfs_result = BfsResult {
            start_user: start_node.clone(),
            end_user: end_node.clone(),
            path_nodes: path_names,
            distance: path_ids.len().saturating_sub(1),
            path_ids,
        };
        serde_json::to_writer_pretty(File::create("bfs_result.json")?, &bfs_result)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }

    // Save graph too (assuming you added write_json_pretty impl as before)
    g.write_json_pretty("graph.json", /*directed=*/false)?;

    Ok(())
}