// Vars are immutable by default
// Rust is a block-scoped language

pub fn run() {
    //Mutability
    let name = "Tom";
    let mut age = 21;
    println!("His name is {} he is {}", name, age);
    age += 1;
    println!("His name is {} he is {}", name, age);

    //Define consts
    const ID: i32 = 001;
    println!("Id: {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("Tom", 21);
    println!("His name is {} he is {}", my_name, my_age);

    //Variable shadowing
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    //Casting
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
}
