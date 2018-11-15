/*
cargo test --test e0071
cargo test --test e0071 with_error -- --nocapture
cargo test --test e0071 without_error1 -- --nocapture

*/

/*
You tried to use structure-literal syntax to create an item that is not a structure or enum variant.
*/
// I lot of problems with recursion_limit
#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        //        type U32 = u32;
        //        let t = U32 { value: 4 }; // error: expected struct, variant or union type,
        //        // found builtin type `u32`
    }
    #[test]
    fn without_error1() {
        enum Foo {
            FirstValue(i32),
        }

        fn main() {
            let _u = Foo::FirstValue(0i32);

            let _t = 4;
        }

        main();
    }
}
