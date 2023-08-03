enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_av(m: Movement) {
    match m {
        Movement::Up => println!("Up"),
        Movement::Down => println!("Down"),
        Movement::Left => println!("Left"),
        Movement::Right => println!("Right"),
    }
}

pub fn run() {
    move_av(Movement::Up);
    move_av(Movement::Up);
    move_av(Movement::Down);
    move_av(Movement::Left);
    move_av(Movement::Left);
    move_av(Movement::Right);
}
