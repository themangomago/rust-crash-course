// Functions
fn takes_anything<T>(x: T) { // x has type T, T is a generic type
}

fn takes_two_of_the_same_things<T>(x: T, y: T) { // Both x and y has the same type
}

fn takes_two_things<T, U>(x: T, y: U) { // Multiple types
}

// Structs
struct Point<T> {
    x: T,
    y: T,
}

pub fn run() {
    let point_a = Point { x: 0, y: 0 }; // T is a int type
    let point_b = Point { x: 0.0, y: 0.0 }; // T is a float type
}
