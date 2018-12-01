/*
A binary operation was attempted on a type which doesn't support it.
*/

// cargo test --test e0365 with_error1 -- --nocapture
// cargo test --test e0365 without_error1 -- --nocapture
// cargo test --test e0368 without_error2 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        //        let mut x = 12f32; // error: binary operation `<<` cannot be applied to
        //        //        type `f32`
        //
        //        x <<= 2;
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        let mut _x = 12u32; // the `u32` type does implement the `ShlAssign` trait

        _x <<= 2; // ok!
    }

    /*
    String concatenation appends the string on the right to the string on the left and may require
    reallocation. This requires ownership of the string on the left. If something should be added
    to a string literal, move the literal to the heap by allocating it with to_owned() like
    in "Your text".to_owned().
    */

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}

}
