#![feature(coerce_unsized)]

/*
A struct with more than one field containing an unsized type cannot implement CoerceUnsized.
This only occurs when you are trying to coerce one of the types in your struct to another type
in the struct. In this case we try to impl CoerceUnsized from T to U which are both types that
the struct takes. An unsized type is any type that the compiler doesn't know the length or
alignment of at compile time. Any struct containing an unsized type is also unsized.
*/

// cargo test --test e0365 with_error1 -- --nocapture
// cargo test --test e0365 without_error1 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {

        //        use std::ops::CoerceUnsized;
        //
        //        struct Foo<T: ?Sized, U: ?Sized> {
        //            a: i32,
        //            b: T,
        //            c: U,
        //        }
        //
        //        // error: Struct `Foo` has more than one unsized field.
        //        impl<T, U> CoerceUnsized<Foo<U, T>> for Foo<T, U> {}
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        use std::ops::CoerceUnsized;

        struct Foo<T: ?Sized> {
            a: i32,
            b: T,
        }

        impl<T, U> CoerceUnsized<Foo<U>> for Foo<T> where T: CoerceUnsized<U> {}

        fn coerce_foo<T: CoerceUnsized<U>, U>(t: T) -> Foo<U> {
            Foo { a: 12i32, b: t } // we use coercion to get the `Foo<U>` type we need
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
