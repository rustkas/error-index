/*
cargo test --test e0073
cargo test --test e0073 with_error -- --nocapture
cargo test --test e0073 without_error1 -- --nocapture

*/

/*

*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        #[allow(dead_code)]
        struct Foo {
            x: Box<Foo>,
        } // error

        // let _foo = Foo{x: Box<Foo>};
        //        let _foo = Foo { x: None };
        let _foo = Foo { x: None };
    }
    #[test]
    fn without_error1() {
        #[allow(dead_code)]
        struct Foo {
            x: Option<Box<Foo>>,
        }
    }
}
