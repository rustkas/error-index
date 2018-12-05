/*
A type parameter which references Self in its default value was not specified.
*/

// cargo test --test e0389 with_error1 -- --nocapture
// cargo test --test e0393 without_error1 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        //        trait A<T = Self> {}
        //
        //        fn together_we_will_rule_the_galaxy(son: &A) {}
        //        // error: the type parameter `T` must be explicitly specified in an
        //        //        object type because its default value `Self` references the
        //        //        type `Self`
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        trait A<T = Self> {}

        fn together_we_will_rule_the_galaxy(_son: &A<i32>) {} // Ok!
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error3() {}
}
