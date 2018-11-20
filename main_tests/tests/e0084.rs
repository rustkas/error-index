#![feature(repr_simd)]
/*
cargo test --test e0075
cargo test --test e0075 with_error -- --nocapture
cargo test --test e0075 without_error1 -- --nocapture

*/

/*
Enum discriminants are used to differentiate enum variants stored in memory. This error indicates
that the same value was used for two or more variants, making them impossible to tell apart.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        #[allow(dead_code)]
        //        #[repr(i32)]
        //        enum NightsWatch {} // error: unsupported representation for zero-variant enum
        enum NightsWatch {}
    }
    #[test]
    fn without_error1() {
        #[allow(dead_code)]
        #[repr(i32)]
        enum NightsWatch {
            JonSnow,
            Commander,
        }
    }
}
