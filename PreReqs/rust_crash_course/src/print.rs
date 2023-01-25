// Define a public function named run that can be called from the main.rs file.

pub fn run() {
    // Print to console
    println!("This line of text is from the print.rs file.");

    // Basic formatting
    println!("Hi my name is {}, and I like to play {}.", "Ben", "guitar");

    // Positional Arguments
    println!("{0} am enjoying learning Rust, but {0} also wish it were easier--kind of like {1}.", "I", "Python" );

    // Named Arguments
    println!("To a novice developer, {framework} looks confusing until you really get your hands on it. Then, it's like {adjective}.", framework = "CosmWasm", adjective = "magic");    

    // Placeholder traits
    println!(" For the number 23 -- Binary: {:b} Hex: {:x} Octal: {:o}", 23, 23, 23);

    // Placeholder for debug trait
    println!("{:?}", (2, false, "goodbye"));

    // Math
    println!("The answer to 11 + 11 is {}", 11 + 11);
}
