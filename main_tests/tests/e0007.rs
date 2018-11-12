/*
cargo test --test e0007
cargo test --test e0007 with_error
cargo test --test e0007 without_error1
cargo test --test e0007 without_error2
*/

/*
Patterns used to bind names must be irrefutable, that is, they must guarantee that a name will be
extracted in all cases.
*/

// book/second-edition/ch18-03-pattern-syntax.html?highlight=@,match#a--bindings

/*
See also E0303
*/
#[cfg(test)]
mod tests {

    #[test]

    fn with_error() {
        //        Code like the following is invalid as it requires the entire Option<String> to be moved into
        //        a variable called op_string while simultaneously requiring the inner String to be moved
        //        into a variable called s.

        let x: Option<String> = Some("s".to_string());
        //TODO uncomment the block code below

        //        match x {
        //
        //            op_string @ Some(s) => {}, // error: cannot bind by-move with sub-bindings
        //            None => {},
        //        }

        //TODO comment the block code below

        match Some(x) {
            Some(ref s) => {
                let _op_string_ref = &Some(s);
                // ...
            }
            None => {}
        }
    }

    #[test]
    fn without_error1() {
        let x: Option<String> = Some("s".to_string());
        match Some(x) {
            Some(ref s) => {
                let _op_string_ref = &Some(s);
                // ...
            }
            None => {}
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
