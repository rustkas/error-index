/*
The Sized trait is a special trait built-in to the compiler for types with a constant size known
at compile-time. This trait is automatically implemented for types as needed by the compiler,
and it is currently disallowed to explicitly implement it for a type.
*/
// cargo test --test e0309 with_error1
// cargo test --test e0309 without_error1
#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        struct Foo {};
        //  impl Sized for Foo {}
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {}

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
