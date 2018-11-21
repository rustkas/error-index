#![feature(intrinsics)]
/*
cargo test --test e0094
cargo test --test e0094 with_error -- --nocapture
cargo test --test e0094 without_error1 -- --nocapture

*/

/*
You gave an invalid number of type parameters to an intrinsic function.

All intrinsic functions are defined
in librustc_codegen_llvm/intrinsic.rs and in libcore/intrinsics.rs in the Rust source code.
*/

#[cfg(test)]
mod tests {
    #[test]
    fn with_error1() {
        extern "rust-intrinsic" {
            //   fn size_of<T, U>() -> usize; // error: intrinsic has wrong number
            //        of type parameters        fn foo(); // error: unrecognized intrinsic function: `foo`
        }
    }

    #[test]
    fn without_error1() {
        use std::cmp::PartialOrd;
        extern "rust-intrinsic" {
            #[allow(dead_code)]
            fn size_of<T>() -> usize; // ok!
        }

        unsafe {
            let value = size_of::<u8>();
            println!("{}", value);
            let value = size_of::<u16>();
            println!("{}", value);
            let value = size_of::<u32>();
            println!("{}", value);
            let value = size_of::<u64>();
            println!("{}", value);
        }
    }
}
