pub fn run() {
    let age = 18;
    let checkId = false;

    //If/Else
    if age >= 21 && checkId {
        println!("Bartender: Drink");
    } else if age < 21 && checkId {
        println!("Bartender: Leave");
    } else {
        println!("need to see your id")
    }
}
