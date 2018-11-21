/*
You can only define an inherent implementation for a type in the same crate where the type
was defined.
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]
    fn with_error1() {

        // impl Vec<u8> { } // error

        //        type Bytes = Vec<u8>;
        //
        //        impl Bytes { } // error, same as above

    }

    #[allow(dead_code)]
    #[test]
    fn without_error1() {
        //  define a trait that has the desired associated functions/types/constants and implement
        // the trait for the type in question
        //  define a new type wrapping the type and define an implementation on the new type

        #[derive(Clone)]
        struct MyVec {
            vec: Vec<bool>,
        }
    }

}
