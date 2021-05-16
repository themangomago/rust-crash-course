// Arrays - fixed list where elements are the same data type

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("{:?}", numbers);

    // Re-assign value
    numbers[2] = 20;

    //Array length
    println!("Array length: {}", numbers.len());

    //Array size
    println!("Array size: {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);

    //Get single value
    println!("{}", numbers[0]);
}
