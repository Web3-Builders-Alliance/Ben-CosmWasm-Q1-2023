// Reference Pointers point directly to a resource in memory
// Arrays are primitive data types
// Vectors are non-primitive data types

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Arrays 1 and 2 values: {:?}", (arr1, arr2));

    // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value.
    // You'll need to use a reference (&) to point to the resource.

    // Vector
    let vec1 = vec![2, 4, 6];
    let vec2 = &vec1;

    println!("Vectors 1 and 2 Values: {:?}", (&vec1, vec2));
}
