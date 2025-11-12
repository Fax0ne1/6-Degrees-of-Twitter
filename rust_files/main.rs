//Main Logic entry point file. Most of all the logic is now handled in the rust_to_python file
//thought that there would be more in this file when initally creating, but makes it a little cleaner in my opnion

//other files to use (in the same directory)
mod graph; 
mod dkj; //probably switching to BFS?, so not used anymore
mod py_rust_com;
mod inputs;
mod json_parser; //not used
mod rust_to_python;

//external libraries and other files (From <file> import <class/function> in python)
use rust_to_python::pipeline;

//main entry point logic for the running of the code
fn main() {
    pipeline();  //shows as a warning because I'm not doing anything with the return value. It still works

}
