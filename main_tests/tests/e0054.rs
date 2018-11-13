/*
cargo test --test e0054
cargo test --test e0054 with_error -- --nocapture
cargo test --test e0054 without_error1 -- --nocapture

*/

/*
It is not allowed to cast to a bool.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        let x = 5;

        // Not allowed, won't compile
        // TODO uncomment code below
        // let x_is_nonzero = x as bool;
        let x_is_nonzero = x != 0;
        println!("{}", x_is_nonzero);
    }
    #[test]
    fn without_error1() {
        let x = 5;

        // Ok
        let x_is_nonzero = x != 0;
        println!("{}", x_is_nonzero);
    }
}
