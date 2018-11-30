/*
An associated const was implemented when another trait item was expected.
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
        trait Foo {
            type N;
        }

        struct Bar;

        //        impl Foo for Bar {
        //            const N : u32 = 0;
        //            // error: item `N` is an associated const, which doesn't match its
        //            //        trait `<Bar as Foo>`
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
        struct Bar;

        trait Foo {
            type N;
        }

        impl Foo for Bar {
            type N = u32; // ok!
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
