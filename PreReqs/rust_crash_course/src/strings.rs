// String types in Rust


/* 
Rust has two string types: String and &str. 
The String type is allocated on the heap and is growable, while &str is a fixed-length String that is usually used for String slices. 
The String type is implemented as a wrapper over a Vec<u8> (a vector of u8 values, also known as bytes).
*/

pub fn run() {
    // String
    let mut hello = String::from("Hello ");

    // Get the length of the String using the .len() method
    println!("Character length of the String 'Hello ': {}", hello.len());

    // print byte capacity of the String using the .capacity() method
    println!("Byte Capacity: {}", hello.capacity());

    // Push a char to the end of the String using the .push() method
    hello.push('W');

    // Push an entire String to the end of the String using the .push_str() method
    hello.push_str("orld!");

    // Print the altered String
    println!("{}", hello);

    // Print the byte capacity of a String using the .capacity() method
    println!("Byte Capacity: {}", hello.capacity());

    // Check if the String is empty using the .is_empty() method
    println!("Is Empty: {}", hello.is_empty());

    // print a boolean if the String contains a certain subString using the .contains() method
    println!("Contains 'World' ?: {}", hello.contains("World"));

    // Replace a subString with another subString using the .replace() method
    println!("Replace World with There: {}", hello.replace("World", "There"));

    // Loop through String by whitespace using the .split_whitespace() method
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create a String with a certain capacity in bytes using the .with_capacity() method 
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assert that some scalar value of your String is true using the assert_eq!() macro and a mathematical expression
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    // Print the s String
    println!("{}", s);
}