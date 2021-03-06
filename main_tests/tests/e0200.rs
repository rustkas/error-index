/*
Unsafe traits must have unsafe implementations. This error occurs when an implementation for an unsafe
trait isn't marked as unsafe. This may be resolved by marking the unsafe implementation as unsafe.
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
        //       struct Foo;
        //
        //      unsafe trait Bar { }
        //
        //    // this won't compile because Bar is unsafe and impl isn't unsafe
        //        impl Bar for Foo { }
        // this will compile
        // safe impl Bar for Foo { }

    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        struct Foo;

        unsafe trait Bar {}

        // this will compile
        unsafe impl Bar for Foo {}
    }

}
