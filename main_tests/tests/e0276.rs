/*
This error occurs when a bound in an implementation of a trait does not match the bounds specified
in the original trait.

*/
// cargo test --test e0276 without_error1 -- --nocapture
#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        //        trait Foo {
        //            fn foo<T>(x: T);
        //        }
        //
        //        impl Foo for bool {
        //            fn foo<T>(x: T) where T: Copy {}
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        trait Foo {
            fn foo<T>(x: T);
        }

        impl Foo for bool {
            fn foo<T>(_x: T) {}
        }
    }
}
