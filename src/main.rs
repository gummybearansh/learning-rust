// cargo add chrono
// add external crate to use their functions
use chrono::{Local};

fn main (){
    let now = Local::now();

    println!("current time {}", now);
}
