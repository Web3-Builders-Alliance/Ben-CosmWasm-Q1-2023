// command line interface commands using cargo

use std::env;

pub fn run() {
    // define a vector of arguments collected from the command line input
    let args: Vec<String> = env::args().collect();

    println!("Arguments: {:?}", args);

    let command = args[2].clone();
    let name = "Jim";

    println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else {
        println!("Ok, bye!");
    }
}
