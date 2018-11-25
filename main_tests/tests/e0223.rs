/*
An attempt was made to retrieve an associated type, but the type was ambiguous.

*/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
        //        trait MyTrait {type X; }
        //
        //        fn main() {
        //            let foo: MyTrait::X;
        //        }

        //The problem here is that we're attempting to take the type of X from MyTrait.
        // Unfortunately, the type of X is not defined, because it's only made concrete
        // in implementations of the trait.
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        trait MyTrait {
            type X;
        }
        struct MyStruct;

        impl MyTrait for MyStruct {
            type X = u32;
        }

        let _foo: <MyStruct as MyTrait>::X;
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error2() {}

}
