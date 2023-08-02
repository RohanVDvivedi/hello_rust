pub fn run() {
    let mut count = 0;
    
    // infinite loop
    loop {
        if !(count < 20) {
            break;
        }
        println!("{}", count);
        count += 1;
    }

    // same as above but with while
    count = 0;
    while count < 20 {
        println!("{}", count);
        count += 1;
    }
}