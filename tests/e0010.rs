#![feature(box_syntax)]

/*
cargo test --test e0010
cargo test --test e0010 with_error
cargo test --test e0010 with_error2
cargo test --test e0010 without_error1
cargo test --test e0010 without_error2
*/

/*
The value of statics and constants must be known at compile time, and they live for the entire
lifetime of a program. Creating a boxed value allocates memory on the heap at runtime,
and therefore cannot be done at compile time. Erroneous code example:
*/


#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {

// TODO: uncomment below
        // const CON : Box<i32> = box 0;
        //  static CON : Box<i32> = box 0;
    }


    #[test]
    fn without_error1() {
        let _con : Box<i32> = box 0;
    }


}
