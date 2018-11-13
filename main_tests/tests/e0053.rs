
/*
cargo test --test e0053
cargo test --test e0053 with_error -- --nocapture
cargo test --test e0053 without_error1 -- --nocapture

*/

/*
The parameters of any trait method must match between a trait implementation and the trait definition.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        //TODO uncomment code below
        //        trait Foo {
        //            fn foo(x: u16);
        //            fn bar(&self);
        //        }
        //
        //        struct Bar;
        //
        //        impl Foo for Bar {
        //            // error, expected u16, found i16
        //            fn foo(x: i16) { }
        //
        //            // error, types differ in mutability
        //            fn bar(&mut self) { }
        //        }
    }
    #[test]
    fn without_error1() {
        trait Foo {
            fn foo(x: u16);
            fn bar(&self);
        }

        struct Bar;

        impl Foo for Bar {
            // error, expected u16, found i16
            fn foo(_x: u16) {}

            // error, types differ in mutability
            fn bar(&self) {}
        }
        let bar = Bar {};
        Bar::foo(9);
        bar.bar();
    }
}
