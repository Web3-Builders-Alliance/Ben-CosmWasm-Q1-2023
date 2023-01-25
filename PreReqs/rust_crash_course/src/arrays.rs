// An array is a list of elements of the same data type, and it is fixed -- meaning its size cannot be altered after it is declared.

// import mem from standard library
use std::mem;

// define function
pub fn run() {
  let mut numbers: [i32; 6] = [2, 4, 6, 8, 10, 12];

  // Re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);

  // Get array length
  println!("Array Length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);
}