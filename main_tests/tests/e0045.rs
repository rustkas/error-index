#![feature(unboxed_closures)]

/*
cargo test --test e0045
cargo test --test e0045 with_error -- --nocapture
cargo test --test e0045 without_error1 -- --nocapture

*/

/*
Rust only supports variadic parameters for interoperability with C code in its FFI.
As such, variadic parameters can only be used with functions which are using the C ABI.
*/

#[cfg(test)]
mod tests {

   #[test]
    fn with_error(){


 //      extern "rust-call" { fn foo(x: u8); }

// or

      // fn foo(x: u8) {}
   }

    #[test]
    fn without_error1(){
//        extern "C" {
//            fn foo (x: u8, ...);
//        }
    }
}
