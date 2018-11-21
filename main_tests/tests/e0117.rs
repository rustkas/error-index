/*
This error indicates a violation of one of Rust's orphan rules for trait implementations.
The rule prohibits any implementation of a foreign trait (a trait defined in another crate) where

the type that is implementing the trait is foreign
all of the parameters being passed to the trait (if there are any) are also foreign.
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]
    fn with_error1() {

        //        impl Drop for u32 {
        //            fn drop(&mut self) {
        //                unimplemented!()
        //            }
        //        }

    }

    #[allow(dead_code)]
    #[test]

    //  ensure that at least one local type is referenced by the impl
    fn without_error1() {
        pub struct Foo; // you define your type in your crate

        impl Drop for Foo {
            // and you can implement the trait on it!
            fn drop(&mut self) {
                unimplemented!()
            }
        }

        impl From<Foo> for i32 {
            // or you use a type from your crate as
            // a type parameter
            fn from(_i: Foo) -> i32 {
                0
            }
        }

        //TODO: how to using From trait implementation for i32?
        //        let value:Foo = Foo;
        //        let i32_value = Foo::from(0);
        //        assert_eq!(0, i32_value);
    }
    #[allow(dead_code)]
    #[test]

    //  define a trait locally and implement that
    // cargo test --test e0117 without_error2 -- --nocapture
    fn without_error2() {
        trait Bar {
            fn get11(&self) -> usize;
        }

        impl Bar for u32 {
            fn get11(&self) -> usize {
                0
            }
        }

        //TODO: how to use Bar functionality in u32?
        let value = 170u32;
        println!("{}", value.get11());
    }

}
