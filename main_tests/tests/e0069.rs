/*
cargo test --test e0069
cargo test --test e0069 with_error -- --nocapture
cargo test --test e0069 without_error1 -- --nocapture

*/

/*
The compiler found a function whose body contains a return; statement but whose return type is not ().
*/
// I lot of problems with recursion_limit
#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        //        // error
        //        fn foo() -> u8 {
        //            return;
        //        }
    }
    #[test]
    fn without_error1() {
        // error
        fn foo() -> u8 {
            return 0;
        }
        let foo = foo();
        println!("{}", foo);
    }
}
