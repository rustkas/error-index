/*
This error indicates the use of a loop keyword (break or continue) outside of a loop.
Without a loop to break out of or continue in, no sensible action can be taken.
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        //        fn some_func() {
        //            break; // error: `break` outside of loop
        //        }

    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]

    fn without_error1() {
        fn some_func() {
            for _ in 0..10 {
                break; // ok!
            }
        }
    }

}
