/*
The types of any associated constants in a trait implementation must match the types
in the trait definition.
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
        //        trait Foo {
        //            const BAR: bool;
        //        }
        //
        //        struct Bar;
        //
        //        impl Foo for Bar {
        //            const BAR: u32 = 5; // error, expected bool, found u32
        //        }
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
    fn without_error1() {
        trait Foo {
            const BAR: u32;
        }

        struct Bar;

        impl Foo for Bar {
            const BAR: u32 = 5; // error, expected bool, found u32
        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        struct Bar;

        trait Foo {
            const N: u32;
        }

        impl Foo for Bar {
            const N: u32 = 0; // ok!
        }
    }
}
