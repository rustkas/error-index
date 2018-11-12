// #![allow(unused)]

/*
cargo test --test e0029
cargo test --test e0029 with_error -- --nocapture
cargo test --test e0029 without_error1 -- --nocapture

*/

/*
n a match expression, only numbers and characters can be matched against a range.
*/

#[cfg(test)]
mod tests {


    static STRING:&'static str = "salutations !";
    #[test]
    fn with_error() {


// The ordering relation for STRINGs can't be evaluated at compile time,
// so this doesn't work:

        match STRING {
            //TODO uncomment the line below
            //"hello" ..= "world" => {},
            "hello" => {println!("{}", "1")},
            "salutations !" => {println!("{}", "2")},
            _ => {}
        }
    }

    #[test]
    fn without_error1() {
// This is a more general version, using a guard:
        match STRING {
            s if s >= "hello" && s <= "world" => {}
            _ => {println!("{}", "Hello, Rust!");}
        }
    }


}
