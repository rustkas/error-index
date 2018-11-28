/*
Patterns used to bind names must be irrefutable. That is, they must guarantee that a name will be
extracted in all cases. Instead of pattern matching the loop variable, consider using a match
or if let inside the loop body.
*/
// cargo test --test e0282 without_error2 -- --nocapture
#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        //        let xs: Vec<Option<i32>> = vec![Some(1), None];
        //
        //        // This fails because `None` is not covered.
        //        for Some(x) in xs {
        //            // ...
        //        }
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        let xs: Vec<Option<i32>> = vec![Some(1), None];

        for item in xs {
            match item {
                Some(_x) => {}
                None => {}
            }
        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        let xs: Vec<Option<i32>> = vec![Some(1), None];

        for item in xs {
            if let Some(_x) = item {
                // ...
            }
        }
    }
}
