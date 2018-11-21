/*
cargo test --test e0075
cargo test --test e0075 with_error -- --nocapture
cargo test --test e0075 without_error1 -- --nocapture

*/

/*
Enum discriminants are used to differentiate enum variants stored in memory. This error indicates
that the same value was used for two or more variants, making them impossible to tell apart.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        //        enum Enum {
        //            X = (1 << 500),
        //            Y = (1 / 0)
        //        }

    }
    #[test]
    fn without_error1() {
        #[allow(dead_code)]
        enum Enum {
            X = 0,
            Y = 1,
        }
    }
}
