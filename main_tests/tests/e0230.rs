#![feature(on_unimplemented)]

/*
The #[rustc_on_unimplemented] attribute lets you specify a custom error message for when
a particular trait isn't implemented on a type placed in a position that needs that trait.

*/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
        //        fn foo<T: Index<u8>>(x: T){}
        //
        //        #[rustc_on_unimplemented = "the type `{Self}` cannot be indexed by `{Idx}`"]
        //        trait Index<Idx> { /* ... */ }
        //
        //        foo(true); // `bool` does not implement `Index<u8>`
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]

    fn without_error1() {
        //        fn foo<T: Index<u32>>(x: T){}
        //
        //        #[rustc_on_unimplemented = "the type `{Self}` cannot be indexed by `{Idx}`"]
        //        trait Index<Idx> { /* ... */ }
        //
        //        foo(1u32);
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]

    fn without_error2() {}
}
