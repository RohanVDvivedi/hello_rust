pub fn run () {
    greeting("Hello", "Rohan");
    let a : i32 = 32;
    let b : i32 = 64;
    println!("{} + {} = {}", a, b, add(a,b));
    let c : i32 = 4;
    let cadd = |n1 : i32, n2 : i32| -> i32 { return n1 + n2 + c;};
    println!("cadd({},{}) = {}", a, b, cadd(a,b));
}

fn greeting(greet : &str, name : &str) {
    println!("{} !! , {}", greet, name);
}

fn add(n1 : i32, n2 : i32) -> i32 {
    return n1 + n2;
}