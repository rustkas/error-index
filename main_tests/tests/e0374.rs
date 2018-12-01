#![feature(coerce_unsized)]

/*
A struct without a field containing an unsized type cannot implement CoerceUnsized.
 An unsized type is any type that the compiler doesn't know the length or alignment of at
 compile time. Any struct containing an unsized type is also unsized.
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
        //        struct Foo<T: ?Sized> {
        //            a: i32,
        //        }
        //
        //        // error: Struct `Foo` has no unsized fields that need `CoerceUnsized`.
        //        impl<T, U> CoerceUnsized<Foo<U>> for Foo<T>
        //            where T: CoerceUnsized<U> {}
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

        // We don't need to impl `CoerceUnsized` here.
        struct Foo {
            a: i32,
        }

        // We add the unsized type field to the struct.
        struct Bar<T: ?Sized> {
            a: i32,
            b: T,
        }

        // The struct has an unsized field so we can implement
        // `CoerceUnsized` for it.
        impl<T, U> CoerceUnsized<Bar<U>> for Bar<T> where T: CoerceUnsized<U> {}
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
