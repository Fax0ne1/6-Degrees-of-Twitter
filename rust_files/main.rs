//Main Logic entry point file.

//other files to use (in the same directory)
mod graph;
mod dkj; //probably switching to BFS?
mod py_rust_com;
mod inputs;
mod json_parser;
mod rust_to_python;

//external libraries and other files (From <file> import <class/function> in python)
use rust_to_python::pipeline;

//main entry point logic for the running of the code
fn main() {
 pipeline();  

}
