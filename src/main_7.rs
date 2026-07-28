// Q. write a function that reads contents of a file 
use std::fs;

// the Result Enum let's you return either `Ok` or `Err` Value
// the Result enum is how you can do error handling in rust
fn main (){
    // returns Result Enum <String, Error>
    // file should exist in the root of the project (not the src)
    let file = fs::read_to_string("file.txt");

    match file {
        Ok(data) => println!("{}", data),
        Err(error) => println!("Error whie reading the file, {}", error)
    }
}
