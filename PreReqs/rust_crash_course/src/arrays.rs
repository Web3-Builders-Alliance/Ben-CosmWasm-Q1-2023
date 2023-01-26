/* An array is a list of elements of the same data type.
It is of fixed length -- meaning its size cannot be altered after it is declared.
*/

// import mem from standard library
use std::mem;

// define function
pub fn run() {
    let mut numbers: [i32; 6] = [7, 2, 19, 22, 37, 53];

    // print numbers array
    println!("Original array: {:?}", numbers);

    // Re-assign the third element's value
    numbers[2] = 20;

    // use debug trait to print numbers array
    println!("Array with reassigned 3rd elements: {:?}", numbers);

    // Get single val
    println!("Single Value of the first element: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated using referenced numbers array
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // create a slice from the referenced numbers array
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}
