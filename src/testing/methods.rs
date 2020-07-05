// https://doc.rust-lang.org/rustdoc/documentation-tests.html

pub fn show_testing() {
    println!("add_one(3) == {}", add_one(3));
    println!("Consider also running cargo test, you can play around with the code in the comment");
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// use librustplaygroundcli::testing::methods::add_one;
/// let arg = 5;
/// let answer = add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let arg = 5;
        let answer = add_one(arg);

        assert_eq!(6, answer);
    }
}
