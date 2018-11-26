#![feature(libc)]
/*
The name chosen for an external crate conflicts with another external crate that has been
imported into the current module.
*/
extern crate foo;
#[cfg(test)]
mod tests {

    //    extern crate core;
    //    extern crate libc as core;

    extern crate core;
    extern crate libc as other_name;

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
