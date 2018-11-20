#![feature(repr_simd)]
/*
cargo test --test e0075
cargo test --test e0075 with_error -- --nocapture
cargo test --test e0075 without_error1 -- --nocapture

*/

/*
Too many type arguments were supplied for a function.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        #[allow(dead_code)]
        fn foo<T>() {}

        //            foo::<f64, bool>(); // error: wrong number of type arguments:
        //        expected 1, found 2
    }
    #[test]
    fn without_error1() {
        fn foo<T>() {}

        foo::<bool>(); // error: wrong number of type arguments:
                       //        expected 1, found 2
    }
}
