pub fn run () {
    greeting("Hello", "Rohan");
}

fn greeting(greet : &str, name : &str) {
    println!("{} !! , {}", greet, name);
}