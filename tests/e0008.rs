/*
cargo test --test e0008
cargo test --test e0008 with_error
cargo test --test e0008 with_error2
cargo test --test e0008 without_error1
cargo test --test e0008 without_error2
*/

/*
Names bound in match arms retain their type in pattern guards. As such, if a name is bound by move 
in a pattern, it should also be moved to wherever it is referenced in the pattern guard code. 
Doing so however would prevent the name from being available in the body of the match arm.
*/

// book/second-edition/ch18-03-pattern-syntax.html?highlight=@,match#a--bindings

/*
See also E0303
*/
#[cfg(test)]
mod tests {

    #[test]

    fn with_error() {
        match Some("hi".to_string()) {
            Some(
            // TODO comment this code to reproduce an error
                ref
                s
            ) if s.len() == 0 => {}, // use s.
            _ => {},
        }
    }

    #[test]
    fn with_error2() {
        #[derive(Clone, Copy)]
        struct A{}

        impl A {
            fn consume(self) -> usize {
                0
            }
        }

            let a = Some(A{});
            match a{
                Some(ref y) if y.clone().consume() > 0 => {}
                _ => {}
            }

    }
    #[test]
    fn without_error1() {
        match Some("hi".to_string()) {
            Some(ref s) if s.len() == 0 => {}, // use s.
            _ => {},
        }
    }

    #[test]
    fn without_error2() {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3...7,
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10...12 } => println!("Found an id in another range"),
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }

}
