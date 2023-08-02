
pub fn run() {
    // variables hold primitive data or references to data
    // variables are immutable by default
    // rust is block scoped language
    let name = "Rohan";
    let mut age = 26; // default to i32
    let weight = 73.5; // default to f64
    let trades = true; // default to bool
    println!("My name is {}, aged {}, weighs {} and trades {}", name, age, weight, trades);
    age += 1;
    println!("My name is {}, aged {}", name, age);


    // primitive types of rust are 
    // i8 u8 to i128 u128 -> i -> int and u -> unsigned int, with explicity number of bits
    // f32, f64
    // bool, char
    // tuples and arrays -> discussed later

    // rust is statically typed langauge i.e. all types known at compile time, but there is string strict type inference

    // explicit typing with range
    let asu_id : i64 = 1224063958;
    println!("And my ASU id is {}, which is in range [{}, {}]", asu_id, std::i64::MIN, std::i64::MIN);

    let a : i64 = 1288690;
    let b : i64 = 138;
    println!("a({}) < b({}) => {}", a, b, a < b);

    // strings
    // primitive str = Immutable fixed-length somewhere in memory
    // String = growable, heap allocated data structure - use when you need to modify or own string data

    let hello1 = "Hello"; // -> primitive str
    let mut hello2 = String::from(hello1); // -> String
    hello2.push(' '); // push a char
    hello2.push_str("World"); // push a primitive str

    println!("h1 {} and h2 {}, length {}, capacity {}", hello1, hello2, hello2.len(), hello2.capacity());
}