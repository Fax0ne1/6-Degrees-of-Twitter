//crates to use cli arguments
use std::{
    io::Write,
    process::{Command, Stdio},
};

//function to run the python script, reading in from stdout w/ python
pub fn run_py_output(script: &str)-> String{
    //run the script and captue the outout
    let output = Command::new("python3") //launch python as a subprocess
        .arg(script)
        .output() //collect stdout/err
        .expect("Failed to Run");

    //if a failure
    if !output.status.success(){
        eprintln!("Python script failed:\n{}", String::from_utf8_lossy(&output.stderr)); //convert from bytes to human readable
        std::process::exit(1);
    }

    //converting the output to bytes
    println!("Data gathered sucessfully");

    // Convert bytes to a string and return
    String::from_utf8_lossy(&output.stdout).into_owned()
}

//funciton to read 'post' to python script using stdin
pub fn run_py_input(script: &str, input_data: &str)->String{
    //similar logic as above, spawn python as a subprocess
    let mut child = Command::new("python3")
        .arg(script)
        .stdin(Stdio::piped()) //allow the writing to python
        .stdout(Stdio::piped()) //capture output of python
        .spawn()
        .expect("Failed");

    //writing the conditions to python's stdin
    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        writeln!(stdin, "{}",input_data).expect("Failed write to stdin");
        
    }//signal EOF to python

    //read the output (stdout)
    let output = child.wait_with_output().expect("Failed to read stdout");

    //check stdout
    if !output.status.success(){
        eprintln!("Python failed:\n{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
    
    //return the value
    String::from_utf8_lossy(&output.stdout).into_owned()

    /*
    Example usage (in main):
        let conditions = r#"{"temperature": 22.5, "humidity": 45}"#;
        let result = run_py_with_input("process_conditions.py", conditions);
        println!("Python returned:\n{}", result);
     */
}


/* 
//run C code function, same subproccess logic as python function
fn run_c(script: &str)->String{
    //spawn the process. Add .arg(arguments after new if need arguments)
    let output = Command::new(script).output().expect("failed to run C exe");

    //check the status
    if output.status.success(){
        //return
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout))
    }else{
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        //return
        String::from_utf8_lossy(&output.stderr).to_string() //no ';' means it is getting returned
    }
    /* Example Usage:

    let result = run_c("./my_c_program");
    println!("Result: {}", result); 

    */
} 
*/

