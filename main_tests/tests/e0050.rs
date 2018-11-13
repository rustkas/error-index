/*
cargo test --test e0050
cargo test --test e0050 with_error -- --nocapture
cargo test --test e0050 without_error1 -- --nocapture

*/

/*
This error indicates that an attempted implementation of a trait method has the wrong number of function parameters.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        trait Foo {
            fn foo(&self, x: u8) -> bool;
        }
        #[warn(dead_code)]
        struct Bar;

        // error: method `foo` has 1 parameter but the declaration in trait `Foo::foo`
        // has 2

        // TODO uncomment code below
        //       impl Foo for Bar {
        //           fn foo(&self) -> bool { true }
        //       }
    }
    #[test]
    fn without_error1() {
        trait Foo {
            fn foo(&self, x: u8) -> bool;
        }
        #[warn(dead_code)]
        struct Bar;

        // error: method `foo` has 1 parameter but the declaration in trait `Foo::foo`
        // has 2
        impl Foo for Bar {
            fn foo(&self, x: u8) -> bool {
                true
            }
        }
    }
}
