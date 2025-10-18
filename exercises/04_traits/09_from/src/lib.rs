// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

// Wrapper Type
pub struct WrappingU32 {
    pub value: u32,
}

// Implement From<u32> for wrapper
impl std::convert::From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

// Demonstrate both conversion styles
fn example() {
    // using Into (via 'into()' method)
    let wrapping1: WrappingU32 = 42.into();

    // Using From directly
    let wrapping2 = WrappingU32::from(42);

    // optional - printing the results
    println!("wrapping1.value = {}", wrapping1.value);
    println!("wrapping2.value = {}", wrapping2.value);
}

fn main() {
    example();
}
