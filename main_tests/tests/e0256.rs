/*
You can't import a type or module when the name of the item being imported is the same as another
type or submodule defined in the module.
*/
extern crate foo;
#[cfg(test)]
mod tests {

    //    use tests::foo::Bar; // error
    //
    //    type Bar = u32;
    //
    //    mod foo {
    //        pub mod Bar { }
    //    }

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
