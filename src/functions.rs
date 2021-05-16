mod age {
    fn calc_age(a1: u8, a2: u8) -> u8 {
        return a1 + a2;
    }

    pub fn get_age(a1: u8, a2: u8) -> u8 {
        calc_age(a1, a2)
    }
}

use age::get_age as other_function;

pub fn run() {
    greeting("hello", "tom");

    let sum = add(5, 5);

    //Closure
    let n3 = 10;
    let add_num = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("sum {}", add_num(3, 3));

    println!("{} ", age::get_age(21, 23));
    println!("{} ", other_function(21, 23));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}
