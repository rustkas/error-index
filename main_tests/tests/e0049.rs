/*
cargo test --test e0049
cargo test --test e0049 with_error -- --nocapture
cargo test --test e0049 without_error1 -- --nocapture

*/

/*
This error indicates that an attempted implementation of a trait method has the wrong number of type parameters.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        // TODO uncomment code below
        //       trait Foo {
        //           fn foo<T: Default>(x: T) -> Self;
        //       }
        //
        //       struct Bar;
        //
        //       // error: method `foo` has 0 type parameters but its trait declaration has 1
        //// type parameter
        //       impl Foo for Bar {
        //           fn foo(x: bool) -> Self { Bar }
        //       }
    }

    #[test]
    fn without_error1() {}
}
