/*
Safe traits should not have unsafe implementations, therefore marking an implementation for a safe
trait unsafe will cause a compiler error. Removing the unsafe marker on the trait noted in
the error will resolve this problem.
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
//        struct Foo;
//
//        trait Bar { }
//
//        // this won't compile because Bar is safe
//        unsafe impl Bar for Foo { }

    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        struct Foo;

        trait Bar { }

        // this won't compile because Bar is safe
        impl Bar for Foo { }

    }

}
