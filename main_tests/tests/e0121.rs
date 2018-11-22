/*
In order to be consistent with Rust's lack of global type inference, type placeholders
are disallowed by design in item signatures.
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]

    fn with_error1() {
        //        fn foo() -> _ { 5 } // error, explicitly write out the return type instead
        //
        //        static BAR: _ = "test"; // error, explicitly write out the type instead
    }
    #[allow(dead_code)]
    #[test]

    fn without_error1() {
        fn foo() -> u8 {
            5
        } // error, explicitly write out the return type instead

        static BAR: &str = "test"; // error, explicitly write out the type instead
    }

}
