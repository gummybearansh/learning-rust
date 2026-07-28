struct User {
    first_name: String,
    last_name: String,
    age: i32
}

fn main (){
    let user = User {
        first_name: String::from("Ansh"),
        last_name: String::from("Lachhwani"),
        age: 32,
    };

    println!("{}", user.first_name);
}
