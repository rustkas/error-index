/*
This error indicates that type inference did not result in one unique possible type, and extra
information is required. In most cases this can be provided by adding a type annotation.
Sometimes you need to specify a generic type parameter manually.
*/
// cargo test --test e0282 without_error2 -- --nocapture
#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        //let _x = "hello".chars().rev().collect();

    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {
        //        struct Foo<T> {
        //            num: T,
        //        }
        //
        //        impl<T> Foo<T> {
        //            fn bar() -> i32 {
        //                0
        //            }
        //
        //            fn baz() {
        //                let number = Foo::bar();
        //            }
        //        }

    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        let _x: Vec<char> = "hello".chars().rev().collect();
        let _x: Vec<_> = "hello".chars().rev().collect();
        let _x = "hello".chars().rev().collect::<Vec<char>>();
        let _x = "hello".chars().rev().collect::<Vec<_>>();
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        struct Foo<T> {
            num: T,
        }

        impl<T> Foo<T> {
            fn bar() -> i32 {
                0
            }

            fn baz() {
                let _number = Foo::<T>::bar();
            }
        }
    }
}
