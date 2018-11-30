#![feature(optin_builtin_traits)]
/*
A cross-crate opt-out trait was implemented on something which wasn't a struct or enum type.
*/
/*
Only structs and enums are permitted to impl Send, Sync, and other opt-out trait, and the struct or
enum must be local to the current crate.
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
        struct Foo;

        impl !Sync for Foo {}

        //        unsafe impl Send for &'static Foo {}
        // error: cross-crate traits with a default impl, like `core::marker::Send`,
        //        can only be implemented for a struct/enum type, not
        //        `&'static Foo`
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
        struct Foo;

        impl !Sync for Foo {}
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
