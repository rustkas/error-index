/*
This error occurs when the compiler doesn't have enough information to unambiguously choose
an implementation.
*/
// cargo test --test e0282 without_error2 -- --nocapture
#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        //        trait Generator {
        //            fn create() -> u32;
        //        }
        //
        //        struct Impl;
        //
        //        impl Generator for Impl {
        //            fn create() -> u32 { 1 }
        //        }
        //
        //        struct AnotherImpl;
        //
        //        impl Generator for AnotherImpl {
        //            fn create() -> u32 { 2 }
        //        }
        //
        //            let _cont: u32 = Generator::create();
        //            // error, impossible to choose one of Generator trait implementation
        //            // Impl or AnotherImpl? Maybe anything else?

    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        trait Generator {
            fn create() -> u32;
        }

        struct AnotherImpl;

        impl Generator for AnotherImpl {
            fn create() -> u32 {
                2
            }
        }

        let _gen1 = AnotherImpl::create();

        // if there are multiple methods with same name (different traits)
        let _gen2 = <AnotherImpl as Generator>::create();
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
