
/*
Declaring certain lifetime names in parameters is disallowed. For example, because the 'static
lifetime is a special built-in lifetime name denoting the lifetime of the entire program
*/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {

//        // error, invalid lifetime parameter name `'static`
//        fn foo<'static>(x: &'static str) { }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]

    fn without_error1() {

        // error, invalid lifetime parameter name `'static`
        fn foo<'a>(x: &'a str) { }
    }

}
