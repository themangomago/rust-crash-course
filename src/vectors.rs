// Vectors - resizeable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Re-assign value
    numbers[2] = 20;

    //Add to vector
    numbers.push(6);
    numbers.push(7);

    //Pop off last element
    numbers.pop();

    //Mutate value
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    //Vector length
    println!("Vector length: {}", numbers.len());

    //Vector size
    println!("Vector size: {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);

    //Get single value
    println!("{}", numbers[0]);
}
