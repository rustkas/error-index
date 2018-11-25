/*
Attempt was made to import an unimportable value. This can happen when trying to import a method
from a trait.
It's invalid to directly import methods belonging to a trait or concrete type.

*/
extern crate foo;
#[cfg(test)]
mod tests {

    mod foo {
        pub trait MyTrait {
            fn do_something() {}
        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        //        use foo::MyTrait::do_something;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]

    fn without_error1() {
        use tests::foo::MyTrait;
        struct MyStruct {}
        impl MyTrait for MyStruct {};

        let _v = MyStruct {};
        MyStruct::do_something();
    }

}
