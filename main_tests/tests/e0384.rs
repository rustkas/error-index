/*
This error occurs when an attempt is made to reassign an immutable variable.
*/

// cargo test --test e0365 with_error1 -- --nocapture
// cargo test --test e0382 without_error2 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        //        let _x = 3;
        //        _x = 5; // error, reassignment of immutable variable

    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        let mut _x = 3;
        _x = 5;
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
