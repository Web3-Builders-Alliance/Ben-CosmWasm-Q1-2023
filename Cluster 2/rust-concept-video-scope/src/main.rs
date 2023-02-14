fn main() {
    println!("This program and associated video is dedicated to explaining the concept of scope in Rust. I will compare that to how scope works in Python, as I'm most familiar with Python.");

    println!("The answer to the meaning of life is {}", MEANING_OF_LIFE);

    fn inner_scope() {
        { // this first bracket begins the scope of the inner_scope function
            let inner1 = 5; // this variable is only available within the scope of the inner_scope function
            println!("The value of inner1 is: {}", inner1);

            inner_inner_scope() // calls the inner_inner_scope function
        } // this second bracket ends the scope of the inner_scope function and inner1 is no longer available

        fn inner_inner_scope() {
            let inner2 = 10; // this variable is only available within the scope of the inner_inner_scope function
            println!("The value of inner2 is: {}", inner2);
        } // this bracket ends the scope of the inner_inner_scope function and inner2 is no longer available
    }

    inner_scope(); // this calls the inner_scope function and prints the value of x. this function is callable in any scope within the main function, and could even be placed before the inner_scope function was even written

// inner_inner_scope();
} // this bracket ends the scope of the main function


pub const MEANING_OF_LIFE: i32 = 42; // this variable is available to all functions in the program because it is defined as a constant

