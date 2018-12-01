#![feature(coerce_unsized)]

/*
The type you are trying to impl CoerceUnsized for is not a struct. CoerceUnsized can only be
implemented for a struct. Unsized types are already able to be coerced without an implementation
of CoerceUnsized whereas a struct containing an unsized type needs to know the unsized type field
it's containing is able to be coerced. An unsized type is any type that the compiler doesn't know
the length or alignment of at compile time. Any struct containing an unsized type is also unsized.
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
        //            a: T,
        //        }
        //
        //        // error: The type `U` is not a struct
        //        impl<T, U> CoerceUnsized<U> for Foo<T> {}
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

        struct Foo<T> {
            a: T,
        }

        // The `Foo<U>` is a struct so `CoerceUnsized` can be implemented
        impl<T, U> CoerceUnsized<Foo<U>> for Foo<T> where T: CoerceUnsized<U> {}
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
