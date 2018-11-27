/*
A lifetime name cannot be declared more than once in the same scope
*/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {

        //        // error, lifetime name `'a` declared twice in the same scope
        //        fn foo<'a, 'b, 'a>(x: &'a str, y: &'b str) { }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]

    fn without_error1() {
        // error, lifetime name `'a` declared twice in the same scope
        fn foo(x: &str, y: &str) {}
    }

}
