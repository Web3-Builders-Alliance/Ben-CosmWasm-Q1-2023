// Varibales in Rust hold primitive data or references to data.
// Variables are immutable by default, and only pertain to the scope of the block in which they are declared.

pub fn run() {
    // define an immutable variable
    let name = "James";

    // define a mutable variable
    let mut age = 163;

    println!("Hi, my name is {} and I am {} years old.", name, age);

    // re-assign the mutable age variable's value
    age = 164;

    println!("Next year I will be {} years old.", age);

    // define a constant variable
    const ID: &str = "007";

    println!("Agent {}.", ID);

    // define multiple variables
    let (last, first_last) = ("Bond", "James Bond");

    println!("The name is {}... {}.", last, first_last);
}
