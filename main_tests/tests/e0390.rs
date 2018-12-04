/*
You tried to implement methods for a primitive type.
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
        //        struct Foo {
        //            x: i32
        //        }
        //
        //        impl *mut Foo {}
        // error: only a single inherent implementation marked with
        //        `#[lang = "mut_ptr"]` is allowed for the `*mut T` primitive
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        struct Foo {
            x: i32,
        }

        trait Bar {
            fn bar();
        }

        impl Bar for *mut Foo {
            fn bar() {} // ok!
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
