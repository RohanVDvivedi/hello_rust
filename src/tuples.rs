// tuples group together values, with max 12 elements

pub fn run() {
    let t : (&str, &str, i8) = ("Rohan", "Gujarat", 26);

    println!("{:?}", t);
    println!("{} is from {}, aged {}", t.0, t.1, t.2);

}