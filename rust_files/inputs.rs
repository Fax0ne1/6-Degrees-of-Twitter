//File for input functions
//Not used in the final example. 

use std::io::{self, Write};
use std::collections::HashSet;

//function to add users to a vector and return it
pub fn add_users() -> Vec<String> {
    let mut names = Vec::new();
    //loop until done, error checking handled in the the function below
    loop {
        let name = get_input("Enter user name: ");
        names.push(name);

        if !ask_yes_no("Do you want to enter another user? (y/n): ") {
            break;
        }
    }
    //return the vector
    names
}

//function to handle error checking
pub fn ask_yes_no(prompt: &str) -> bool {
    //loop until valid input
    loop {
        let answer = get_input(prompt);
        let answer = answer.to_lowercase().trim().to_string();
        //checking for blank
        if answer.is_empty() {
            println!("Input cannot be blank. Please enter 'y' or 'n'.");
            continue;
        }
        //similar to a switch statement, will return true or false based on input
        match answer.as_str() {
            "y" | "yes" => return true,
            "n" | "no"  => return false,
            _ => {
                println!("Invalid input. Please type 'y' or 'n'.");
                continue;
            }
        }
    }
}

//fucntion to gather user input 
pub fn get_input(prompt: &str) -> String {
    
    loop {
        //flush causes immediate print and unwrap says panic if fail
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let trimmed = input.trim();

        //checking for blank names
        if !trimmed.is_empty() {
            return trimmed.to_string();
        } else {
            println!("Input cannot be blank. Please try again.");
        }
    }
}

//function to check for duplicates in the usernames
//need to pass the hashset and user name return true or false based on that.
fn appened_names(hashes: &mut HashSet<String>, name: &str) -> bool{
    hashes.insert(name.to_string())
}