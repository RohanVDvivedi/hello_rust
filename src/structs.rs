
// Traditional c style struct
struct Person1
{
    name: String,
    age: u8,
}

impl Person1 {
    fn new(name : &str, age : u8) -> Person1 {
        return Person1 {
            name: String::from(name),
            age: age,
        };
    }
}

// Tuple struct
struct Person2(String, u8);

pub fn run() {
    let mut p1 = Person1::new("Rohan", 26);
    println!("p1 = person named {} aged {}", p1.name, p1.age);
    p1.age += 1;
    println!("p1 = person named {} aged {}", p1.name, p1.age);

    let mut p2 = Person2 (
        String::from("Brohan"),
        26,
    );
    println!("p2 = person named {} aged {}", p2.0, p2.1);
    p2.1 += 1;
    println!("p2 = person named {} aged {}", p2.0, p2.1);
}