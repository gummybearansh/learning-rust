// Q. write a function that returns index of first 'a' in a string 

// the Option enum lets you return `Some` value or `None` value 
// mostly used to handle None values in rust 
fn find_first_a (s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            // returning Some variant of the Option Enum
            return Some(index as i32)
        }
    }

    // didn't find an index - return None
    return None;
}

fn main (){
    let index = find_first_a(String::from("preet"));
    
    // now i can just pattern match on the index
    match index {
        Some(value) => println!("Found a at {}", value),
        None => println!("Did not find 'a'")
    }
}
