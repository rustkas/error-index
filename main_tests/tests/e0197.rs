/*
Inherent implementations (one that do not implement a trait but provide methods associated with a type)
are always safe because they are not implementing an unsafe trait. Removing the unsafe keyword from
the inherent implementation will resolve this error.
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
        //        struct Foo;
        //
        //        // this will cause this error
        //        unsafe impl Foo { }
        //        // converting it to this will fix it
        //        impl Foo { }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        struct Foo;

        // this will cause this error
        impl Foo {}
        // converting it to this will fix it
        impl Foo {}
    }
}
