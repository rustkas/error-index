#![feature(coerce_unsized)]

/*
Trait methods cannot be declared const by design. For more information, see RFC 911.
https://github.com/rust-lang/rfcs/pull/911
*/

// cargo test --test e0365 with_error1 -- --nocapture
// cargo test --test e0365 without_error1 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
