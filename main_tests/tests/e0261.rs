/*
When using a lifetime like 'a in a type, it must be declared before being used.
*/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {

        //        // error, use of undeclared lifetime name `'a`
        //        fn foo(x: &'a str) { }
        //
        //        struct Foo {
        //            // error, use of undeclared lifetime name `'a`
        //            x: &'a str,
        //        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]

    fn without_error1() {
        fn foo(_x: &str) {}

        struct Foo<'a> {
            x: &'a str,
        }
    }

}
