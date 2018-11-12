/*
cargo test --test e0013
cargo test --test e0013 with_error
cargo test --test e0013 without_error1

*/

/*
Static and const variables can refer to other const variables. But a const variable cannot refer to
a static variable
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        const _X: i32 = 4;
        // TODO: uncomment below
        // static X: i32 = 42;
        const _Y: i32 = _X;
    }

    #[test]
    fn without_error1() {
        const _X: i32 = 4;
        static _X1: i32 = _X;
        const _Y: i32 = _X;
    }

}
