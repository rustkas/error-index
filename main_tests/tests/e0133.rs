/*
cargo test --test e0133
cargo test --test e0133 with_error
cargo test --test e0133 without_error1


*/

/*
Unsafe code was used outside of an unsafe function or block.
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]
    fn with_error() {
        unsafe fn f() {
            return;
        } // This is the unsafe code

        // TODO uncomment this
        // f(); // error: call to unsafe function requires unsafe function or block
    }

    /*
    Using unsafe functionality is potentially dangerous and disallowed by safety checks. Examples:

    Dereferencing raw pointers
    Calling functions via FFI
    Calling functions marked unsafe

    These safety checks can be relaxed for a section of the code by wrapping the unsafe instructions
    with an unsafe block
    */
    #[test]
    fn without_error1() {
        unsafe fn f() {
            return;
        }

        unsafe {
            f();
        } // ok!
    }

}
