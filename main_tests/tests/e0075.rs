#![feature(repr_simd)]
/*
cargo test --test e0075
cargo test --test e0075 with_error -- --nocapture
cargo test --test e0075 without_error1 -- --nocapture

*/

/*
The #[simd] attribute can only be applied to non empty tuple structs, because it doesn't make sense
to try to use SIMD operations when there are no values to operate on.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        //        #[repr(simd)]
        //        struct Bad;

    }
    #[test]
    fn without_error1() {
        #[repr(simd)]
        #[allow(dead_code)]
        struct Good(u32);
    }
}
