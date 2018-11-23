#![feature(lang_items)]

/*
A lang item was redefined.
*/
/*
You can build a free-standing crate by adding #![no_std] to the crate attributes:

ⓘ
#![no_std]Run
See also https://doc.rust-lang.org/book/first-edition/no-stdlib.htm
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]
    // no error
    fn with_error() {
        //        #[lang = "panic_impl"]
        //        fn foo(){}
        //
        //
        // struct Foo; // error: duplicate lang item found: `panic_impl`
    }

    #[allow(dead_code)]
    #[test]
    fn without_error1() {}

    #[allow(dead_code)]
    #[allow(unused_unsafe)]
    #[test]
    fn without_error2() {}

}
