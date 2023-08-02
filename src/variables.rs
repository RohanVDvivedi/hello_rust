// variables hold primitive data or references to data
// variables are immutable by default
// rust is block scoped language

pub fn run() {
    let name = "Rohan";
    let mut age = 26;
    println!("My name is {}, aged {}", name, age);
    age += 1;
    println!("My name is {}, aged {}", name, age);
}