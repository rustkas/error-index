#![feature(intrinsics)]
/*
cargo test --test e0092
cargo test --test e0092 with_error -- --nocapture
cargo test --test e0092 without_error1 -- --nocapture

*/

/*
You declared an unknown intrinsic function.
Please check you didn't make a mistake in the function's name. All intrinsic functions are defined
in librustc_codegen_llvm/intrinsic.rs and in libcore/intrinsics.rs in the Rust source code.
*/

#[cfg(test)]
mod tests {
    #[test]
    fn with_error1() {
        //        extern "rust-intrinsic" {
        //            fn foo(); // error: unrecognized intrinsic function: `foo`
        //        }
        //
        //        unsafe {
        //            foo();
        //        }
    }

    #[test]
    fn without_error1() {
        extern "rust-intrinsic" {
            #[allow(dead_code)]
            fn atomic_fence(); // ok!
        }

        unsafe {
            atomic_fence();
        }
    }
}
