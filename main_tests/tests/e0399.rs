#![feature(associated_type_defaults)]

/*
You implemented a trait, overriding one or more of its associated types but did not reimplement
its default methods.
*/

// cargo test --test e0398 with_error1 -- --nocapture
// cargo test --test e0398 without_error1 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        //        pub trait Foo {
        //            type Assoc = u8;
        //            fn bar(&self) {}
        //        }
        //
        //        impl Foo for i32 {
        //            // error - the following trait items need to be reimplemented as
        //            //         `Assoc` was overridden: `bar`
        //            type Assoc = i32;
        //        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        pub trait Foo {
            type Assoc = u8;
            fn bar(&self) {}
        }

        impl Foo for i32 {
            type Assoc = i32;
            fn bar(&self) {} // ok!
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error3() {}
}
