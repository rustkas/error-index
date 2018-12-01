/*
Private items cannot be publicly re-exported. This error indicates that you attempted to pub use
a type or value that was not itself public.
*/

// cargo test --test e0309 with_error1
// cargo test --test e0364 without_error1
#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        //        mod foo {
        //            const X: u32 = 1;
        //        }
        //
        //        pub use foo::X;

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
        pub mod foo {
            pub const X: u32 = 1;
        }

        println!("{}", foo::X);
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
