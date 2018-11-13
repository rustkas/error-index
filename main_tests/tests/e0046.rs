/*
cargo test --test e0046
cargo test --test e0045 with_error -- --nocapture
cargo test --test e0045 without_error1 -- --nocapture

*/

/*
Items are missing in a trait implementation.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        trait Foo {
            fn foo();
        }

        struct Bar;

        // impl Foo for Bar {}
        // error: not all trait items implemented, missing: `foo`
    }

    #[test]
    fn without_error1() {
        trait Foo {
            fn foo();
        }

        struct Bar;

        impl Foo for Bar {
            fn foo() {} // ok!
        }
    }
}
