/*
Assignments are not allowed in pattern guards, because matching cannot have side effects.
*/
// cargo test --test e0282 without_error2 -- --nocapture
#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        //        match Some(()) {
        //            None => { },
        //            option if { option = None; false } => { },
        //            Some(_) => { } // When the previous match failed, the option became `None`.
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
        match Some(()) {
            None => {}
            option if option != None => {}
            Some(_) => {} // When the previous match failed, the option became `None`.
        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
