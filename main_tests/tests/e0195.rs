/*
Your method's lifetime parameters do not match the trait declaration.
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
//        trait Trait {
//            fn bar<'a,'b:'a>(x: &'a str, y: &'b str);
//        }
//
//        struct Foo;
//
//        impl Trait for Foo {
//            fn bar<'a,'b>(x: &'a str, y: &'b str) {
//                // error: lifetime parameters or bounds on method `bar`
//                // do not match the trait declaration
//            }
//        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        trait Trait {
            fn t<'a,'b:'a>(x: &'a str, y: &'b str);
        }

        struct Foo;

        impl Trait for Foo {
            fn t<'a,'b:'a>(_x: &'a str, _y: &'b str) { // ok!
            }
        }
    }
}
