/*
cargo test --test e0004
cargo test --test e0004 with_error
cargo test --test e0004 without_error
*/

/*
This error indicates that the compiler cannot guarantee a matching pattern for one or more possible
inputs to a match expression. Guaranteed matches are required in order to assign values to match
expressions, or alternatively, determine the flow of execution.
*/

#[cfg(test)]
mod tests {

    // This error provide E0004 error message

    #[test]

    fn with_error() {
        enum Terminator {
            HastaLaVistaBaby,
            TalkToMyHand,
        }

        let x = Terminator::HastaLaVistaBaby;

        match x { // error: non-exhaustive patterns: `HastaLaVistaBaby` not covered
            // TODO remove one arm or all of them
            Terminator::TalkToMyHand => {}
        }
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

    }
}
