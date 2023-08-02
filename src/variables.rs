
pub fn run() {
    // variables hold primitive data or references to data
    // variables are immutable by default
    // rust is block scoped language
    let name = "Rohan";
    let mut age = 26; // default to i32
    let weight = 73.5; // default to f64
    println!("My name is {}, aged {}, weighs {}", name, age, weight);
    age += 1;
    println!("My name is {}, aged {}, weighs {}", name, age, weight);


    // primitive types of rust are 
    // i8 u8 to i128 u128 -> i -> int and u -> unsigned int, with explicity number of bits
    // f32, f64
    // bool, char
    // tuples and arrays -> discussed later

    // rust is statically typed langauge i.e. all types known at compile time, but there is string strict type inference
}