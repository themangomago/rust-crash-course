pub fn run() {
    println!("Print from console.");

    // Basic formating
    println!("Integer: {}", 1);
    println!("{} are {}", "Roses", "red");

    //Positional arguments
    println!("{0} are {1}, {0} are not {2}", "roses", "red", "blue");

    //Named arguments
    println!("{flower} are {color}", flower = "roses", color = "red");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x}", 10, 10);

    //Debug traits
    println!("{:?}", (12, true, "test"));

    //Basic math
    println!("{}", 10 + 10);
}
