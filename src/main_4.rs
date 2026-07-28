// Enums let u enumerate various types of an value 
// here i could have direction be strings - but then i could also have a string like "youtube" which doesnt make sense 
// enum let u fix the only valid type that it might have
enum Direction {
    North, 
    East, 
    South, 
    West
}


fn main (){
    let my_direction = Direction::North;
    let new_direction = move_around (my_direction);
}

fn move_around (direction : Direction) -> Direction {
    // logic to move a character around
}

