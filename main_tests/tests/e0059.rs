#![feature(unboxed_closures)]

/*
cargo test --test e0059
cargo test --test e0059 with_error -- --nocapture
cargo test --test e0059 without_error1 -- --nocapture

*/

/*
The built-in function traits are generic over a tuple of the function arguments.
If one uses angle-bracket notation (Fn<(T,), Output=U>) instead of parentheses (Fn(T) -> U)
to denote the function trait, the type parameter should be a tuple. Otherwise function call
notation cannot be used and the trait will not be implemented by closures.
*/
// I lot of problems with recursion_limit
#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        // fn foo1<F: Fn<i32>>(f: F) -> F::Output { f(3) } // error

    }
    #[test]
    fn without_error1() {
        #[allow(dead_code)]
        fn foo2<F: Fn<(i32,)>>(f: F) -> F::Output {
            f(3)
        }
    }
}
