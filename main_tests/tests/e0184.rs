/*
Explicitly implementing both Drop and Copy for a type is currently disallowed.
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]
    fn with_error1() {

        //        #[derive(Copy)]
        //        struct Foo{}
        //        impl Drop for Foo{
        //            fn drop(&mut self) {
        //                println!("Dropping!");
        //            }
        //        }

    }

    #[allow(dead_code)]
    #[test]
    fn without_error1() {
        #[derive(Clone)]
        struct Foo {}
        impl Drop for Foo {
            fn drop(&mut self) {
                println!("Dropping!");
            }
        }
    }
}
