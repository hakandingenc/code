//! Contains functions for converting between units.

// The following is an example of file documentation for Rust.

/// Converts a Fahrenheit temperature into Celsius.
///
/// # Examples
///
/// ```
/// # use convert::*;
/// assert_eq!( f_to_c(212.), 100. );
/// ```
pub fn f_to_c(f: f64) -> f64 {
    // Argument and return types must be specified
    // pub: Public, which enables the function to be imported to other modules
    5. / 9. * (f - 32.)
    //1. Implicit conversion for arithmetic operations of different types are not allowed in Rust
    //2. When the last statement is the return statement, the semicolon is optional (and highly encouraged to be left out).
}

#[cfg(test)] // Run only when in test mode
mod tests {
    use super::f_to_c;

    #[test]
    fn water_boiling() {
        assert_eq!(100., f_to_c(212.));
    }

    #[test]
    fn water_freezing() {
        assert_eq!(0., f_to_c(32.));
    }

    #[test]
    fn same_number() {
        assert_eq!(-40., f_to_c(-40.));
    }
}
