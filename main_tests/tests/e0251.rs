/*
Two items of the same name cannot be imported without rebinding one of the items
under a new local name.

*/
extern crate foo;
#[cfg(test)]
mod tests {

    mod foo {
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub struct baz;
    }

    mod bar {
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub mod baz {}
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        use tests::bar::*;
        use tests::foo::baz; // error, do `use foo::baz as quux` instead on the previous line
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]

    fn without_error1() {}
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    // move the type bindings in the where clause
    fn without_error2() {}
}
