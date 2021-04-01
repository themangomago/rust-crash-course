/*
    Primitives:
    integers: u8 and i8 up to 128
    floats: f32, f64
    char
    bool
    tuples
    arrays

*/

pub fn run() {
    // Default i32
    let x = 1;

    // Default f64
    let y = 2.5;

    // Explicit type
    let z: i64 = 4124;

    //Finx max size
    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);

    //bool
    let is_active: bool = x == 1;

    let a1 = 'a';
    let face = '\u{1f600}';

    println!("{:?}", (x, y, z, is_active, a1, face));
}
