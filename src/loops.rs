pub fn run() {
    let mut count = 0;
    let limit = 20;
    
    // infinite loop
    loop {
        if !(count < limit) {
            break;
        }
        println!("loop : {}", count);
        count += 1;
    }

    // same as above but with while
    count = 0;
    while count < limit {
        println!("while : {}", count);
        count += 1;
    }

    count = 90;

    // for loop
    for count in 0..limit {
        println!("for : {}", count);
    }

    println!("count = {}, count inside loop is different variable", count);
}