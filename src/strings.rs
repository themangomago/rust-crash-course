/*
Primitive str = immutable fixed-length
String = growable, heap-allocated

*/

pub fn run() {
    let mut hello = String::from("hello");

    //Get length
    println!("length: {}", hello.len());

    //Push a char
    hello.push('w');
    println!("length: {}", hello.len());

    //Push a string
    hello.push_str("orld!");

    //Capacity
    println!("capacity: {}", hello.capacity());

    //Check is empty
    println!("is empty: {}", hello.is_empty());

    //Contains substr
    println!("contains substring: {}", hello.contains("world"));

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("capacity: {}", s.capacity());
    s.push_str("asdasasdasds");
    println!("capacity: {}", s.capacity());

    println!("{}", s);

    println!("{}", hello);

    //Assertion
    assert_eq!(14, s.len());
}
