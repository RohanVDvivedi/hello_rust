pub fn run() {
    let age = 16;
    let id = true;
    if (age < 20) && id {
        println!("Will be serving you drinks, but off the receipt");
    } else if (age < 20) && !id {
        println!("No drinks for you");
    } else if age < 40 {
        println!("You must be at work");
    } else if (age < 100) && !id {
        println!("Go get your ID");
    } else {
        println!("Enjoy you retired fellow !!");
    }

    // shorthand
    let a = 56;
    println!("a ({}) > 60 = {}", a, (if a > 60 {"yes"} else {"no"}))
}