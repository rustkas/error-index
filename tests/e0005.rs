/*
cargo test --test e0005
cargo test --test e0005 with_error
cargo test --test e0005 without_error1
cargo test --test e0005 without_error2
*/

/*
Patterns used to bind names must be irrefutable, that is, they must guarantee that a name will be
extracted in all cases.
*/

#[cfg(test)]
mod tests {

    #[test]

    fn with_error() {
        let x = Some(1);
        // TODO uncomment this line below
        // let Some(y) = x;

        if let Some(_y1) = x{}
    }

    #[test]
    fn without_error1() {
        let x = Some(1);

        for _i in 0..3 {
            if let Some(_y1) = x{}
        }

    }

    #[test]
    fn without_error2() {
        let x = Some(1);

        match x {
            Some(_y) => {
                // do something
            },
            None => {}
        }
    }

}
