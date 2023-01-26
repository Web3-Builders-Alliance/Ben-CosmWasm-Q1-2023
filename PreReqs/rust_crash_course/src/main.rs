#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

mod arrays;
mod cli;
mod conditionals;
mod data_types;
mod enums;
mod functions;
mod loops;
mod pointer_refs;
mod print;
mod strings;
mod structs;
mod tuples;
mod variables;
mod vectors;

fn main() {
    println!("Hello! This function is designed to print the text that you are reading to the console. It isn't meant for anything other than to prove that it works as intended, and that I know how to print a line of text. It is from the main function in the main.rs file.");
    println!("I am not commenting anything out, so all of the submodules and their associated run functions will run everytime.");

    print::run();
    variables::run();
    data_types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_refs::run();
    structs::run();
    enums::run();
    cli::run();
}
