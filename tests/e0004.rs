/*
cargo test --test e0004
cargo test --test e0004 with_error
cargo test --test e0004 without_error1
cargo test --test e0004 without_error2
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
        #[allow(dead_code)]
        enum Terminator {
            HastaLaVistaBaby,
            TalkToMyHand,
        }

        let x = Terminator::HastaLaVistaBaby;

        match x {
            // error: non-exhaustive patterns: `HastaLaVistaBaby` not covered
            // TODO remove one arm or all of them
            Terminator::HastaLaVistaBaby => {}
            Terminator::TalkToMyHand => {}
        }
    }

    #[test]
    fn without_error1() {
        #[allow(dead_code)]
        enum Terminator {
            HastaLaVistaBaby,
            TalkToMyHand,
        }

        let x = Terminator::HastaLaVistaBaby;

        match x {
            Terminator::HastaLaVistaBaby => {}
            Terminator::TalkToMyHand => {}
        }
    }

    #[test]
    fn without_error2() {
        #[allow(dead_code)]
        enum Terminator {
            HastaLaVistaBaby,
            TalkToMyHand,
        }

        let x = Terminator::HastaLaVistaBaby;

        match x {
            Terminator::HastaLaVistaBaby => {}
            _ => {}
        }
    }
}
