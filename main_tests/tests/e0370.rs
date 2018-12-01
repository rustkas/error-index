#[deny(overflowing_literals)]
/*
The maximum value of an enum was reached, so it cannot be automatically set in the next enum value
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

//        enum Foo {
//            X = 0x7fffffffffffffff,
//            Y, // error: enum discriminant overflowed on value after
//            //        9223372036854775807: i64; set explicitly via
//            //        Y = -9223372036854775808 if that is desired outcome
//        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
       //please set manually the next enum value or put the enum variant with the maximum value
        // at the end of the enum. Examples:

        enum Foo {
            X = 0x7fffffffffffffff,
            Y = 0, // ok!
        }
    }



    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        enum Foo {
            Y = 0, // ok!
            X = 0x7fffffffffffffff,
        }
    }

}
