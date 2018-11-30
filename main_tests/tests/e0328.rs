#![feature(coerce_unsized)]
//#![feature(unsize)]
/*
The Unsize trait should not be implemented directly. All implementations of Unsize are provided
automatically by the compiler.
*/

/*
If you are defining your own smart pointer type and would like to enable conversion from a sized
to an unsized type with the DST coercion system, use CoerceUnsized instead.
*/

// cargo test --test e0309 with_error1
// cargo test --test e0309 without_error1
#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {

        //        use std::marker::Unsize;
        //
        //        pub struct MyType;
        //
        //        impl<T> Unsize<T> for MyType {}
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        use std::ops::CoerceUnsized;

        pub struct MyType<T: ?Sized> {
            field_with_unsized_type: T,
        }

        impl<T, U> CoerceUnsized<MyType<U>> for MyType<T> where T: CoerceUnsized<U> {}
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        struct Bar;

        trait Foo {
            const N: u32;
        }

        impl Foo for Bar {
            const N: u32 = 0; // ok!
        }
    }
}
