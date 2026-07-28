struct Rect {
    width: i32, 
    height: i32
}

// implementation of Rect (functions attached to the Rect struct)
impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    // functions here must have &self
    fn say_hello(&self) -> String{
        String::from("Hello")
    }

    // does not have &self as parameter 
    // becomes a static function (function called on the Class not the object of the class)
    fn info () {
        println!("i could give u info but i won't"); 
    }
}

fn main (){
    let rectangle = Rect {
        width: 30,
        height: 20
    };

    println!("{}", rectangle.area());
    println!("{}", rectangle.say_hello());
    // static function called directly on the class
    Rect::info();
}
