// #![allow(unused)]

/*
cargo test --test e0023
cargo test --test e0023 with_error
cargo test --test e0023 without_error1
cargo test --test e0023 without_error2

*/

/*
A pattern used to match against an enum variant must provide a sub-pattern for each field of the
enum variant.
This error indicates that a pattern attempted to extract an incorrect number of fields from a variant.
*/

#[cfg(test)]
mod tests {
    enum Fruit {
        Apple(String, String),
        Pear(u32),
    }
    #[test]
    fn with_error() {
        let x = Fruit::Apple(String::new(), String::new());
        let _x = Fruit::Pear(0);
        // Incorrect.
        match x {
            //TODO uncomment the lines below
            //Fruit::Apple(a) => {},
            //Fruit::Apple(a, b, c) => {},
            Fruit::Apple(_a, _b) => {}
            Fruit::Pear(_a) => {}
        }
    }

    #[test]
    fn without_error1() {
        let x = Fruit::Apple(String::new(), String::new());

        // Correct.
        match x {
            Fruit::Apple(_a, _b) => {}
            _ => {}
        }
    }

    #[test]
    fn without_error2() {
        let x = Fruit::Apple(String::new(), String::new());

        // Incorrect.
        match x {
            Fruit::Apple(_a, _b) => {}
            Fruit::Pear(_a) => {}
        }
    }

}
