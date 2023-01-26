// loops in Rust

/*
loops are used to iterate over a collection of values
*/

pub fn run() {
    let mut count = 0;

    // create an infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        // break out of loop once the count reaches 20
        if count == 20 {
            break;
        }
    }

    // While loop -- need to increment the count
    while count <= 100 {
        if count % 3 == 0 && count % 5 == 0 {
            println!("{} is divisible by 3 and 5", count);
        } else if count % 3 == 0 {
            println!("{} is divisible by 3, but not 5", count);
        } else if count % 5 == 0 {
            println!("{} is divisible by 5, but not 3", count);
        } else {
            println!("{} is not divisible by 3 or 5", count);
        }

        // Increment count
        count += 1;
    }

    // For Range -- no need to increment the count
    for x in 0..101 {
        if x % 3 == 0 && x % 5 == 0 {
            println!("{} is divisible by 3 and 5", x);
        } else if x % 3 == 0 {
            println!("{} is divisible by 3, but not 5", x);
        } else if x % 5 == 0 {
            println!("{} is divisible by 5, but not 3", x);
        } else {
            println!("{} is not divisible by 3 or 5", x);
        }
    }
}
