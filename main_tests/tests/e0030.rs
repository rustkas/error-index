// #![allow(unused)]

/*
cargo test --test e0030
cargo test --test e0030 with_error -- --nocapture
cargo test --test e0030 without_error1 -- --nocapture

*/

/*
When matching against a range, the compiler verifies that the range is non-empty
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        //TODO uncomment the lines below
        //        match 5 {
        //
        //            // This range is empty, and the compiler can tell.
        //            6 ..= 5 => {}
        //        }
    }

    #[test]
    fn without_error1() {
        match 1 {
            // This range is empty, and the compiler can tell.
            1..=1 => {
                println!("{}", "1");
            }
            _ => {
                println!("{}", "empty");
            }
        }
    }
    #[test]
    fn without_error10() {
        match 10 {
            // This range is empty, and the compiler can tell.
            1..=1 => {
                println!("{}", "1");
            }
            _ => {
                println!("{}", "empty");
            }
        }
    }
    #[test]
    fn without_error2() {
        let value = 1;
        match value {
            // This range is empty, and the compiler can tell.
            1..=1 => {
                println!("{}", "1");
            }
            _ => {
                println!("{}", "empty");
            }
        }
    }
    #[test]
    fn without_error3() {
        let value = 10;
        match value {
            // This range is empty, and the compiler can tell.
            1..=1 => {
                println!("{}", "1");
            }
            _ => {
                println!("{}", "empty");
            }
        }
    }
}
