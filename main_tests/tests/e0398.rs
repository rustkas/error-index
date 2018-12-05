/*

*/

// cargo test --test e0398 with_error1 -- --nocapture
// cargo test --test e0398 without_error1 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        fn foo(_arg: &Box<Debug>) {
            /* ... */
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        fn foo<'a>(_arg: &'a Box<Debug + 'a>) {
            /* ... */
        }
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
