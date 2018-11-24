/*
This error indicates a violation of one of Rust's orphan rules for trait implementations.
The rule concerns the use of type parameters in an implementation of a foreign trait
(a trait defined in another crate), and states that type parameters must be "covered"
by a local type.

*/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {

        //        use foo::ForeignTrait;
        //
        //        impl<T> ForeignTrait for T { } // error
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error2() {
        use foo::ForeignTrait2;

        struct MyType<T>(T);
        struct MyType2;
        impl<T> ForeignTrait2<T, MyType<T>> for MyType2 {} // error
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        use foo::ForeignTrait;
        struct MyType<T>(T);
        impl<T> ForeignTrait for MyType<T> {} // Ok
                                              // Please note that a type alias is not sufficient.
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error2() {}

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error3() {}

}
