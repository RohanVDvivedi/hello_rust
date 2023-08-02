
pub fn run() {
    // variables hold primitive data or references to data
    // variables are immutable by default
    // rust is block scoped language
    let name = "Rohan";
    let mut age = 26;
    println!("My name is {}, aged {}", name, age);
    age += 1;
    println!("My name is {}, aged {}", name, age);


    // primitive types of rust are 
    // i8 u8 to i128 u128 -> i -> int and u -> unsigned int, with explicity number of bits
    // f32, f64
    // bool, char
    // tuples and arrays
}