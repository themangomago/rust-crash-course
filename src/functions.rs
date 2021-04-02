pub fn run() {
    greeting("hello", "tom");

    let sum = add(5, 5);

    //Closure
    let n3 = 10;
    let add_num = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("sum {}", add_num(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}
