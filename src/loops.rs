pub fn run() {
    let mut count = 0;

    //Infinte loop
    loop {
        count += 1;
        print!("{} ", count);

        if count == 20 {
            break;
        }
    }

    // While
    count = 0;
    while count < 10 {
        count += 1;
        print!("{} ", count);
    }

    //For
    for x in 0..10 {
        print!("{} ", x);
    }
}
