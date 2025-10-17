// pub fn example() {
//     // Trying to get the size of a str (or any other DST)
//     // via `std::mem::size_of` will result in a compile-time error.
//     //
//     // TODO: Comment out the following line and move on to the next exercise.
//     std::mem::size_of::<str>();
// }
//

use std::mem;

pub fn example() {
    let s: &str = "Hello, world!";

    /* 1. Size of the reference itself */
    println!("size_of::<&str>() = {}", mem::size_of::<&str>());

    /* 2. Runtime size of the data that `s` points to */
    // `as_bytes()` gives us a `[u8]` slice, which is also a DST,
    // but `size_of_val` can work with unsized values at runtime.
    let bytes = s.as_bytes();
    println!(
        "Runtime size of string slice (bytes) = {}",
        mem::size_of_val(bytes)
    );

    /* For comparison: the number of bytes in the string */
    println!("String length (len()) = {}", s.len());
}
