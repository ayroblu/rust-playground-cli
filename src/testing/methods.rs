pub fn show_testing() {
    println!("add_one(3) == {}", add_one(3));
    println!("Consider also running cargo test, you can play around with the code in the comment");
}

/// Adds one to the number given.
/// Doesn't actually test cause not lib: https://github.com/rust-lang/rust/issues/50784
///
/// # Examples
///
/// ```
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
