/*
cargo test --test e0009
cargo test --test e0009 with_error
cargo test --test e0009 with_error2
cargo test --test e0009 without_error1
cargo test --test e0009 without_error2
*/

/*
In a pattern, all values that don't implement the Copy trait have to be bound the same way. 
The goal here is to avoid binding simultaneously by-move and by-ref.
*/

// book/second-edition/ch18-03-pattern-syntax.html?highlight=@,match#a--bindings

/*
See also E0303
*/
#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        #[allow(dead_code)]
        struct X { x: (), }

        let x = Some((X { x: () }, X { x: () }));
        match x {
            //TODO uncomment 'ref'
            Some((_y, //ref
            _z)) => {}, // error: cannot bind by-move and by-ref in the
            //        same pattern
            None => panic!()
        }
    }

    #[test]
    fn with_error2() {
        #[allow(dead_code)]
        struct X { x: (), }

        let x = Some((X { x: () }, X { x: () }));
        match x {

            Some((
                     //TODO uncomment 'ref'
                     // ref
                     _y,  _z)) => {}, // error: cannot bind by-move and by-ref in the
            //        same pattern
            None => panic!()
        }
    }
    #[test]
    fn without_error1() {
        #[allow(dead_code)]
        struct X { x: (), }

        let x = Some((X { x: () }, X { x: () }));
        match x {

            Some((_y,  _z)) => {},
            //        same pattern
            None => panic!()
        }
    }

    #[test]
    fn without_error2() {
        #[allow(dead_code)]
        struct X { x: (), }

        let x = Some((X { x: () }, X { x: () }));
        match x {

            Some((ref _y,  ref _z)) => {},
            //        same pattern
            None => panic!()
        }
    }

    #[test]
    /// this solution do not recommended
    fn without_error3() {
        #[allow(dead_code)]
        #[derive(Clone, Copy)]
        struct X { x: (), }

        let x = Some((X { x: () }, X { x: () }));
        match x {

            Some((ref _y,   _z)) => {},
            //        same pattern
            None => panic!()
        }
    }

}
