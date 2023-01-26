// Vectors in Rust

/*
Vectors are resizable arrays, though they can contain multiple data types, just like a tuple.
A vector is an extensible tuple.
*/

// import mem from standard library
use std::mem;

// define function
pub fn run() {
    let mut numbers: Vec<i32> = vec![7, 2, 19, 22, 37, 53];

    // print numbers vector
    println!("Original vector: {:?}", numbers);

    // Re-assign the third element's value
    numbers[2] = 20;

    // add to vector using the .push() method
    numbers.push(5);
    numbers.push(6);
    numbers.push(7);

    // remove last element from vector using the .pop() method
    numbers.pop();

    // loop through vector values using the .iter() method
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate values using the .iter_mut() method, multiply each value by 2
    for x in numbers.iter_mut() {
        *x *= 2;
        // print each mutated value
        println!("Mutated numbers: {}", x);
    }

    // use debug trait to print numbers vector
    println!("Vector with reassigned 3rd elements: {:?}", numbers);

    // Get single val of first element
    println!("Single Value of the first element: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // create a slice from the referenced numbers vector
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}
