/*
cargo test --test e0002
*/

/*
This error indicates that an empty match expression is invalid because the type it is matching
on is non-empty (there exist values of this type). In safe code it is impossible to create
an instance of an empty type, so empty match expressions are almost never desired.
This error is typically fixed by adding one or more cases to the match expression.
*/

#[cfg(test)]
mod tests {

    // This error provide E0004 error message

    #[test]
    //#[ignore]
    fn with_error() {
        fn foo(x: Option<String>) {
            match x {
                // TODO remove line below
                _ => { /* ... */ }
            }
        }
        foo(Some("".to_string()));
    }
    #[allow(dead_code)]
    #[test]
    fn without_error() {
        enum Empty1 {}

        fn foo(x: Empty1) {
            match x {
                // empty
            }
        }
        // it is forbidden
        //        let empty = Empty1;
        //        foo(empty);
    }
}
