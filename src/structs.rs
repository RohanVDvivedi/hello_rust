
// Traditional c style struct
struct Person 
{
    name: String,
    age: u8,
}

pub fn run() {
    let mut p1 = Person {
        name: String::from("Rohan"),
        age: 26,
    };
    println!("p1 = person named {} aged {}", p1.name, p1.age);
    p1.age += 1;
    println!("p1 = person named {} aged {}", p1.name, p1.age);
}