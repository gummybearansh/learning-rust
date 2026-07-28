fn main() {
    // macro (!) at the end 
    // print a dynamic variable {} whose value is the return value of the function call
    // println!("{}", is_even(31));
    // println!("{}", fib(5));

    // one way to create a string 
    let name = String::from("hello world");
    let len = get_str_len(name);
    println!("{}", len);
}

// // takes in 32 bit int as input and returns a bool 
// fn is_even(num: i32) -> bool { 
//     return num % 2 == 0;
// }

// // find the ith fibonacci number
// fn fib(num: i32) -> i32 { 
//     let mut first = 0;
//     let mut second = 1;
//
//     if num == 0 {
//         return first;
//     }
//
//     if num == 1 {
//         return second;
//     }
//
//     for _i in 3..num + 1 {
//         let temp = second;
//         second = second + first;
//         first = temp;
//     }
//
//     return second;
//
// }

// get length of a string 
fn get_str_len(str: String) -> usize {
    // implicit return - if i put semi colon it won't return anything and this will just go away 
    str.chars().count()
}
