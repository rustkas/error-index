/*
You can't import a value whose name is the same as another value defined in the module.
*/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {

        //
        //        use bar::foo; // error: an item named `foo` is already in scope
        //
        //        fn foo() {}
        //
        //        mod bar {
        //            pub fn foo() {}
        //        }

    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]

    fn without_error1() {
        use bar::foo as bar_foo; // ok!

        fn foo() {}

        mod bar {
            pub fn foo() {}
        }
    }

}
