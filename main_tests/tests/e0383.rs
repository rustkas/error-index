/*
This error occurs when an attempt is made to partially reinitialize a structure that is
currently uninitialized.
*/

// cargo test --test e0365 with_error1 -- --nocapture
// cargo test --test e0382 without_error2 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {

        //        struct Foo {
        //            a: u32,
        //        }
        //        impl Drop for Foo {
        //            fn drop(&mut self) { /* ... */ }
        //        }
        //
        //        let mut x = Foo { a: 1 };
        //        drop(x); // `x` is now uninitialized
        //        x.a = 2; // error, partial reinitialization of uninitialized structure `t`

    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        struct Foo {
            a: u32,
        }
        impl Drop for Foo {
            fn drop(&mut self) {
                /* ... */
            }
        }

        let mut x = Foo { a: 1 };
        drop(x);
        x = Foo { a: 2 };
        drop(x);
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
