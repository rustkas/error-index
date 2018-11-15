#![feature(unboxed_closures)]

/*
cargo test --test e0060
cargo test --test e0060 with_error -- --nocapture
cargo test --test e0060 without_error1 -- --nocapture

*/

/*
External C functions are allowed to be variadic. However, a variadic function takes a minimum number
of arguments.
*/
// I lot of problems with recursion_limit
#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        use std::os::raw::{c_char, c_int};

        #[allow(dead_code)]
        extern "C" {
            fn printf(_: *const c_char, ...) -> c_int;

        }

        unsafe {
            printf("hello, C" as *const str as *const i8);
        }
    }
    #[test]
    fn without_error1() {
        use std::os::raw::{c_char, c_int};
        extern "C" {
            fn printf(_: *const c_char, ...) -> c_int;

        }
        unsafe {
            use std::ffi::CString;

            let fmt = CString::new("test\n").unwrap();
            printf(fmt.as_ptr());

            let fmt = CString::new("number = %d\n").unwrap();
            printf(fmt.as_ptr(), 3);

            let fmt = CString::new("%d, %d\n").unwrap();
            printf(fmt.as_ptr(), 10, 5);
        }
    }
}
