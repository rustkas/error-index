/*
cargo test --test e0001 tests::it_works
*/

/*
This error suggests that the expression arm corresponding to the noted pattern will never
be reached as for all possible values of the expression being matched, one of the preceding
patterns will match.
*/
#[cfg(test)]
mod tests {
    #[test]
    //#[should_panic()]
    #[allow(unreachable_patterns)]
    fn with_error() {
        //  #[warn(unreachable_patterns)]
        match Some(0) {
            Some(_bar) => { /* ... */ }
            _x => { /* ... */ } // This handles the `None` case
            _ => { /* ... */ }  // All possible cases have already been handled
        }
    }

    #[test]
    fn without_error() {
        match Some(0) {
            Some(_bar) => { /* ... */ }

            _ => { /* ... */ } // All possible cases have already been handled
        }
    }
}
