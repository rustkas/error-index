/*
This error indicates that a type or lifetime parameter has been declared but not actually used.
*/

// cargo test --test e0389 with_error1 -- --nocapture
// cargo test --test e0389 without_error1 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        //        enum Foo<T> {
        //            Bar,
        //        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {
        //        struct Foo<'a, T> {
        //            x: *const T,
        //        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        enum Foo {
            Bar,
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        enum Foo<T> {
            Bar(T),
        }
    }
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error3() {
        use std::marker::PhantomData;

        struct Foo<'a, T: 'a> {
            x: *const T,
            phantom: PhantomData<&'a T>,
        }
    }
}
