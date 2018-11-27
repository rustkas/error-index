/*
You tried to supply a type which doesn't implement some trait in a location which expected that trait.
*/
// cargo test --test e0276 without_error1 -- --nocapture
#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
//        fn foo<F: Fn(usize)>(x: F) { }
//
//
//            // type mismatch: ... implements the trait `core::ops::Fn<(String,)>`,
//            // but the trait `core::ops::Fn<(usize,)>` is required
//            // [E0281]
//            foo(|y: String| { });


    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {
        fn foo<F: Fn(usize)>(_x: F) { }


        // type mismatch: ... implements the trait `core::ops::Fn<(String,)>`,
        // but the trait `core::ops::Fn<(usize,)>` is required
        // [E0281]
        foo(|_y: usize| { });

    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {

    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {

    }
}
