/*
Inner items do not inherit type parameters from the functions they are embedded in.
*/

// cargo test --test e0398 with_error1 -- --nocapture
// cargo test --test e0398 without_error1 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        //        fn foo<T>(x: T) {
        //            fn bar(y: T) { // T is defined in the "outer" function
        //                // ..
        //            }
        //            bar(x);
        //        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {
        //        fn foo<T>(x: T) {
        //            type MaybeT = Option<T>;
        //            // ...
        //        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error3() {
        //        fn foo<T>(x: T) {
        //            struct Foo {
        //                x: T,
        //            }
        //            // ...
        //        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        fn foo<T>(x: T) {
            let bar = |y: T| {
                // explicit type annotation may not be necessary
                // ..
            };
            bar(x);
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        fn foo<T>(x: T) {
            fn bar<T>(y: T) {
                // ..
            }
            bar(x);
        }
    }
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error3() {
        fn foo<T>(x: T) {
            type MaybeT<T> = Option<T>;
        }
    }
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error4() {
        impl<T> Foo<T> {
            pub fn foo(&self, x: T) {
                self.bar(x);
            }

            fn bar(&self, y: T) {
                // ..
            }
        }
    }
}
