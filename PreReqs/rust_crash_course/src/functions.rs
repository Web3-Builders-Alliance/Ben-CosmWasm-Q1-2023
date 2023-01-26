// Functions store and execute blocks of code

pub fn run() {
    insult("you smell", "funny");

    // Bind function values to variables
    let get_sum = add(11, 12);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 17;
    let add_numbers = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_numbers(11, 12));
}

fn insult(adv: &str, descriptor: &str) {
    println!("Hmmm... {} {}...", adv, descriptor);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 // lines with no ; at the end act as return statements
}
