// Enums are ways to express your code in a descriptive and simple way, where appropriate (ok..)
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let player_direction:Direction = Direction::Up;
    // let player_direction:Direction = Direction::Down;

    match player_direction {
        Direction::Up => println!("We are heading up!"),
        Direction::Down => println!("We are heading down.."),
        Direction::Left => println!("Left is right!"),
        Direction::Right => println!("Moving towards the right!"),
    }
}

// Ok this one I don't really understand, but I have a hunch it will be important!
