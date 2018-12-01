/*
When Trait2 is a subtrait of Trait1 (for example, when Trait2 has a definition like trait Trait2:
Trait1 { ... }), it is not allowed to implement Trait1 for Trait2. This is because Trait2
already implements Trait1 by definition, so it is not useful to do this.
*/

// cargo test --test e0365 with_error1 -- --nocapture
// cargo test --test e0365 without_error1 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        //        trait Foo { fn foo(&self) { } }
        //        trait Bar: Foo { }
        //        trait Baz: Bar { }
        //
        //        impl Bar for Baz { } // error, `Baz` implements `Bar` by definition
        //        impl Foo for Baz { } // error, `Baz` implements `Bar` which implements `Foo`
        //        impl Baz for Baz { } // error, `Baz` (trivially) implements `Baz`
        //        impl Baz for Bar { } // Note: This is OK
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        trait Foo {
            fn foo(&self) {}
        }
        trait Bar: Foo {}
        trait Baz: Bar {}

        impl Baz for Bar {} // Note: This is OK
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
