#![feature(intrinsics)]
/*
cargo test --test e0092
cargo test --test e0092 with_error -- --nocapture
cargo test --test e0092 without_error1 -- --nocapture

*/

/*
You tried to declare an undefined atomic operation function.
*/

#[cfg(test)]
mod tests {
    #[test]
    fn with_error1() {
        //        extern "rust-intrinsic" {
        //            fn atomic_foo(); // error: unrecognized atomic operation
        //            //        function
        //        }
    }

    #[test]
    fn without_error1() {
        extern "rust-intrinsic" {
            #[allow(dead_code)]
            fn atomic_fence(); // ok!
        }
    }
}
