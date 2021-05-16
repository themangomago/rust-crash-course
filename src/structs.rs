//Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

//Tupel struct
struct TColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

//Methods are functions attached to objects
impl Person {
    //Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    //Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("{:?}", (c.red, c.green, c.blue));

    c.red = 200;

    println!("{:?}", (c.red, c.green, c.blue));

    let mut c2 = TColor(255, 0, 0);

    println!("{:?}", (c2.0, c2.1, c2.2));

    let mut p = Person::new("Tom", "Hall");
    println!("Person {}", p.full_name());

    p.set_last_name("Test");
    println!("Person {}", p.full_name());
}
