/*
Attempt was made to import an item whereas an extern crate with this name has already been imported.

*/
extern crate foo;
#[cfg(test)]
mod tests {

    mod foo1 {
        #[allow(non_camel_case_types)]
        pub trait foo {
            fn do_something();
        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        // no error
        use tests::foo1::foo;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]

    fn without_error1() {}

}
