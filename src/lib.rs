//! # mwx_test_cargo
//!
//! `mwx_test_cargo` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = mwx_test_cargo::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = super::add_one(10);
        assert_eq!(result, 11);
    }
}
