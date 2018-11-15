/*
cargo test --test e0067
cargo test --test e0067 with_error -- --nocapture
cargo test --test e0067 without_error1 -- --nocapture

*/

/*
The left-hand side of a compound assignment expression must be a place expression.
*/
// I lot of problems with recursion_limit
#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        //        use std::collections::LinkedList;
        //
        //// Bad: assignment to non-place expression
        //        LinkedList::new() += 1;

        //        fn some_func(i: &mut i32) {
        //            i += 12; // Error : '+=' operation cannot be applied on a reference !
        //        }
        //        let mut x:i32 = 9;
        //        some_func(&mut x);

    }
    #[test]
    fn without_error1() {
        let mut i: i32 = 0;

        i += 12; // Good !

        // ...

        fn some_func(i: &mut i32) {
            *i += 12; // Good !
        }
        some_func(&mut i);
    }
}
