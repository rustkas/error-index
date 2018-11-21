/*
cargo test --test e0075
cargo test --test e0075 with_error -- --nocapture
cargo test --test e0075 without_error1 -- --nocapture

*/

/*
Too few type arguments were supplied for a function
*/

#[cfg(test)]
mod tests {
    #[test]
    fn with_error1() {
        //    fn foo<T, U>() {}
        //
        //    fn main() {
        //        foo::<f64>(); // error: wrong number of type arguments: expected 2, found 1
        //    }
    }

    #[test]
    fn without_error1() {
        fn foo<T, U>(_x: T) {}

        let x: bool = true;
        // foo::<f64>(x);    // error: wrong number of type arguments:
        //        expected 2, found 1
        foo::<_, f64>(x); // same as `foo::<bool, f64>(x)`
    }
}
