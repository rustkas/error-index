/*
cargo test --test e0075
cargo test --test e0075 with_error -- --nocapture
cargo test --test e0075 without_error1 -- --nocapture

*/

/*
You gave too few lifetime arguments
*/

#[cfg(test)]
mod tests {
    #[test]
    fn with_error1() {
        //        fn foo<'a: 'b, 'b: 'a>() {}
        //
        //        foo::<'static>(); // error: wrong number of lifetime arguments:
        //                          //        expected 2, found 1
    }

    #[test]
    fn without_error1() {
        fn foo<'a: 'b, 'b: 'a>() {}

        foo::<'static, 'static>();
    }
}
