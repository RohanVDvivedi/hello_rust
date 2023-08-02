
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
    fn describe(&self) -> String {
        return format!("{} is aged {}.", self.name, self.age);
    }
    fn age_by_an_year(&mut self) {
        self.age += 1;
    }
}

// Tuple struct
struct Person2(String, u8);

impl Person2 {
    fn new(name : &str, age : u8) -> Person2 {
        return Person2 (
            String::from(name),
            age,
        );
    }
    fn describe(&self) -> String {
        return format!("{} is aged {}.", self.0, self.1);
    }
    fn age_by_an_year(&mut self) {
        self.1 += 1;
    }
}

pub fn run() {
    let mut p1 = Person1::new("Rohan", 26);
    println!("p1 = {}", p1.describe());
    p1.age_by_an_year();
    println!("p1 = {}", p1.describe());

    let mut p2 = Person2::new("Brohan", 26);
    println!("p2 = {}", p2.describe());
    p2.age_by_an_year();
    println!("p2 = {}", p2.describe());
}