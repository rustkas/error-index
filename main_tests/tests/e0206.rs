/*
You can only implement Copy for a struct or enum
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
        //        type Foo = [u8; 256];
        //        impl Copy for Foo { } // error
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error2() {
        //        #[derive(Copy, Clone)]
        //        struct Bar;
        //        impl Copy for &'static mut Bar { } // error

    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        struct Foo;
        impl Copy for Foo {}

        impl Clone for Foo {
            fn clone(&self) -> Foo {
                Foo {}
            }
        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error2() {
        #[derive(Copy, Clone)]
        struct Foo;
    }
}
