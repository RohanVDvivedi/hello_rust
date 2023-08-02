pub fn run() {
    let age = 60;
    let id = true;
    if age < 20 {
        println!("No drinks for you");
    } else if age < 40 {
        println!("You must be at work");
    } else if (age < 100) && !id {
        println!("Go get your ID");
    } else {
        println!("Enjoy you retired fellow !!");
    }
}