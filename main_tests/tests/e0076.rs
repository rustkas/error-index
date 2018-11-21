/*
cargo test --test e0075
cargo test --test e0075 with_error -- --nocapture
cargo test --test e0075 without_error1 -- --nocapture

*/

/*
When using the #[simd] attribute to automatically use SIMD operations in tuple struct, the types
in the struct must all be of the same type, or the compiler will trigger this error.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        //        #[repr(simd)]
        //        struct Bad(u16, u32, u32);

    }
    #[test]
    fn without_error1() {
        #[allow(dead_code)]
        #[repr(simd)]
        struct Good(u32, u32, u32);
    }
}
