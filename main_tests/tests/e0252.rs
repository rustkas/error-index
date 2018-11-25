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
        //        use foo::baz;
        //        use bar::baz; // error, do `use bar::baz as quux` instead

    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]

    fn without_error1() {
        use tests::bar::baz;
        use tests::foo::baz as foo_baz; // ok!
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]

    fn without_error2() {
        let _x = foo::baz; // ok!
    }
}
