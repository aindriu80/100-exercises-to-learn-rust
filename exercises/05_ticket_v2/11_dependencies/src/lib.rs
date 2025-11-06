// TODO: Add `anyhow` as a dependency of this project.
//  Don't touch this import!

// When you import a type (`Error`) from a dependency, the import path must start
// with the crate name (`anyhow`, in this case).
use anyhow::{anyhow, Error};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_use_anyhow_error() {
        let e: Error = anyhow!("something went wrong");
        println!("{}", e);
    }
}
