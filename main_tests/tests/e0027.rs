// #![allow(unused)]

/*
cargo test --test e0027
cargo test --test e0027 with_error -- --nocapture
cargo test --test e0027 without_error1 -- --nocapture
cargo test --test e0027 without_error2 -- --nocapture
*/

/*
This error indicates that a pattern for a struct fails to specify a sub-pattern for every one of
the struct's fields
*/

#[cfg(test)]
mod tests {

    struct Dog {
        name: String,
        age: u32,
    }

    #[test]
    fn with_error() {
        let d = Dog {
            name: "Rusty".to_string(),
            age: 8,
        };

        // This is incorrect.
        match d {
            //TODO uncomment the line below
            // Dog { age: x } => {println!("{}",x)}
            Dog { age: x, name: y } => println!("{} {}", x, y),
        }
    }

    #[test]
    fn without_error1() {
        let d = Dog {
            name: "Rusty".to_string(),
            age: 8,
        };

        // This is incorrect.
        match d {
            Dog { age: x, name: y } => println!("{} {}", x, y),
        }
    }

    #[test]
    fn without_error2() {
        let d = Dog {
            name: "Rusty".to_string(),
            age: 8,
        };

        // This is incorrect.
        match d {
            Dog { age: x, .. } => println!("{}", x),
        }
    }
}
