// tuples in Rust

/*
a tupel is a collection of values
unlike an array, these values can be of different types
*/

pub fn run() {
    let person: (&str, &str, i8) = ("Ben", "Earth", 101);

    println!(
        "{} is from {} and is {} years old",
        person.0, person.1, person.2
    );
}
