
/*
The name for an item declaration conflicts with an external crate's name.
https://doc.rust-lang.org/reference.html#statements
*/
extern crate foo;
#[cfg(test)]
mod tests {

    //    extern crate core;
    //
    //    struct core;

    extern crate core as xyz;
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct abc;

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {}

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]

    fn without_error1() {}

}
