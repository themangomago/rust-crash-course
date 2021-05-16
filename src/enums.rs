enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => {
            println!("up")
        }
        Movement::Down => println!("down"),
        Movement::Left => println!("left"),
        _ => println!("right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;

    move_avatar(avatar1);
    move_avatar(avatar2);
}
