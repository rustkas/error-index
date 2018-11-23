/*
There are various restrictions on transmuting between types in Rust; for example types being transmuted
must have the same size. To apply all these restrictions, the compiler must know the exact
types that may be transmuted. When type parameters are involved, this cannot always be done.
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]
    // no error
    fn with_error() {
        use std::mem::transmute;

        struct Foo<T>(Vec<T>);

        fn foo<T>(x: Vec<T>) {
            // we are transmuting between Vec<T> and Foo<F> here
            let _y: Foo<T> = unsafe { transmute(x) };
            // do something with y
        }
    }

    #[allow(dead_code)]
    #[test]
    fn without_error1() {
        use std::mem::transmute;

        struct Foo<T>(Vec<T>);

        trait MyTransmutableType: Sized {
            fn transmute(_: Vec<Self>) -> Foo<Self>;
        }

        impl MyTransmutableType for u8 {
            fn transmute(x: Vec<u8>) -> Foo<u8> {
                unsafe { transmute(x) }
            }
        }

        impl MyTransmutableType for String {
            fn transmute(x: Vec<String>) -> Foo<String> {
                unsafe { transmute(x) }
            }
        }

        // ... more impls for the types you intend to transmute

        fn foo<T: MyTransmutableType>(x: Vec<T>) {
            let _y: Foo<T> = <T as MyTransmutableType>::transmute(x);
            // do something with y
        }
    }

    #[allow(dead_code)]
    #[allow(unused_unsafe)]
    #[test]
    fn without_error2() {
        unsafe {
            // use std::ptr;
            // ptr::read(&v as *const _ as *const SomeType) // `v` transmuted to `SomeType`
        }
    }

}
