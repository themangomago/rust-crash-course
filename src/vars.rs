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
}
