// TODO: this is an example of an orphan rule violation.
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  a foreign type (`u32`, from `std`).
//  Look at the compiler error to get familiar with what it looks like.
//  Then delete the code below and move on to the next exercise.

// impl PartialEq for u32 {
//     fn eq(&self, _other: &Self) -> bool {
//         todo!()
//     }
// }

// Problem with above code ->
// impl PartialEq for u32 violates the orphan rule because both the trait (PartialEq) and the type (u32) are defined outside of your crate.
// There is no local type involved in this impl (the note about “no local type before any uncovered type parameters”).

// If you actually wanted to compare u32 values using a custom equality rule, you'd have to
// wrap the type in a new struct (new‑type pattern) and implement the foreign trait for that wrapper:

#[derive(Debug, Clone, Copy)]
pub struct MyU32(u32);

impl std::cmp::PartialEq for MyU32 {
    fn eq(&self, other: &Self) -> bool {
        // custom logic; for example compare only even/odd status
        self.0 % 2 == other.0 % 2
    }
}
