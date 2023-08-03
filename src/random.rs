use rand::Rng;

pub fn run() {
    let score = rand::thread_rng().gen_range(0..101);
    println!("Your score = {}", score);
    let mut grade = 'F';
    if score > 90 {
        grade = 'A';
    } else if score > 80 {
        grade = 'B';
    } else if score > 70 {
        grade = 'C';
    } else if score > 60 {
        grade = 'D';
    } else if score > 50 {
        grade = 'E';
    }
    println!("And your grade is {}", grade);
}
