//#![recursion_limit="6"]

/*
cargo test --test e0055
cargo test --test e0055 with_error -- --nocapture
cargo test --test e0055 without_error1 -- --nocapture

*/

/*
During a method call, a value is automatically dereferenced as many times as needed to make the
value's type match the method's receiver. The catch is that the compiler will only attempt to
dereference a number of times up to the recursion limit (which can be set via
the recursion_limit attribute).
*/
// I lot of problems with recursion_limit
#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        struct Foo;

        impl Foo {
            fn foo(&self) {}
        }

        let _foo = Foo;
        let ref_foo = &&Foo;

        // error, reached the recursion limit while auto-dereferencing &&Foo
        ref_foo.foo();
    }
    #[test]
    fn without_error1() {}
}
