/*
cargo test --test e0303
cargo test --test e0303 with_error
cargo test --test e0303 without_error1
cargo test --test e0303 without_error2
*/

/*
In certain cases it is possible for sub-bindings to violate memory safety. Updates to the borrow
checker in a future version of Rust may remove this restriction, but for now patterns
must be rewritten without sub-bindings.
*/

// book/second-edition/ch18-03-pattern-syntax.html?highlight=@,match#a--bindings

/*
See also E0303
*/
#[cfg(test)]
mod tests {

    #[test]

    fn with_error() {
        // TODO uncomment comments below
        match Some("hi".to_string()) {
            //            ref op_string_ref @ Some(s) => {},
            ref _op_string_ref => {} // TODO comment this line
                                     //       None => {},
        }
    }

    #[test]
    fn without_error1() {
        match Some("hi".to_string()) {
            ref _op_string_ref => {} // TODO comment this line
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

    #[test]
    fn without_error3() {
        match Some("hi".to_string()) {
            Some(_s) => {}
            None => {}
        }
    }

}
