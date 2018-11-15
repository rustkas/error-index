#![feature(repr_simd)]
/*
cargo test --test e0074
cargo test --test e0074 with_error -- --nocapture
cargo test --test e0074 without_error1 -- --nocapture

*/

/*
When using the #[simd] attribute on a tuple struct, the components of the tuple struct must all be
of a concrete, nongeneric type so the compiler can reason about how to use SIMD with them.
This error will occur if the types are generic.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        #[repr(simd)]
        #[allow(dead_code)]
        struct Bad<T>(T, T, T);

        let _value = Bad(0, 0, 0);
    }
    #[test]
    fn without_error1() {
        #[repr(simd)]
        #[allow(dead_code)]
        struct Good(u32, u32, u32);
    }
}
