pub fn run() {
    println!("Enter your age");
    let mut age: String = String::new();
    std::io::stdin()
        .read_line(&mut age)
        .expect("No input received");
    let mut age: u8 = age.trim().parse().expect("Age provided was not a number");
    age += 1;
    println!("Your age anext year will be {}", age);
}
