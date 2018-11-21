/*
cargo test --test e0075
cargo test --test e0075 with_error -- --nocapture
cargo test --test e0075 without_error1 -- --nocapture

*/

/*
You're trying to write an inherent implementation for something which isn't a struct nor an enum.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        //        impl (u8, u8) {
        //            // error: no base type found for inherent implementation
        //            fn get_state(&self) -> String {
        //                // ...
        //            }
        //        }
    }

    // implement a trait on the type or wrap it in a struct.
    #[test]
    //cargo test --test e0118 without_error1 -- --nocapture
    fn without_error1() {
        // we create a trait here
        trait LiveLongAndProsper {
            fn get_state(&self) -> String;
        }

        // and now you can implement it on (u8, u8)
        impl LiveLongAndProsper for (u8, u8) {
            fn get_state(&self) -> String {
                "He's dead, Jim!".to_owned()
            }
        }

        let value: (u8, u8) = (0u8, 0u8);
        let string = value.get_state();
        println!("{}", string);
    }

    // you can create a newtype. A newtype is a wrapping tuple-struct
    #[test]
    //cargo test --test e0118 without_error2 -- --nocapture
    fn without_error2() {
        struct TypeWrapper((u8, u8));

        impl TypeWrapper {
            fn get_state(&self) -> String {
                "Fascinating!".to_owned()
            }
        }

        let value = TypeWrapper((0u8, 0u8));
        println!("{}", value.get_state());
    }

}
