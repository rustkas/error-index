/*
An attempt was made to implement Drop on a concrete specialization of a generic type.
It is not possible to specialize Drop to a subset of implementations of a generic type.
*/

// cargo test --test e0365 with_error1 -- --nocapture
// cargo test --test e0365 without_error1 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        //        trait Foo{}
        //
        //        struct MyStruct<T> {
        //            t: T
        //        }
        //
        //        impl<T: Foo> Drop for MyStruct<T> {
        //            fn drop(&mut self) {}
        //        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        trait Foo {}

        struct MyStruct<T> {
            t: T,
        }

        struct MyStructWrapper<T: Foo> {
            t: MyStruct<T>,
        }

        impl<T: Foo> Drop for MyStructWrapper<T> {
            fn drop(&mut self) {}
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        trait Foo {}

        struct MyStruct<T> {
            t: T,
        }

        struct MyStructWrapper<T: Foo> {
            t: MyStruct<T>,
        }

        impl<T> Drop for MyStructWrapper<T>
        where
            T: Foo,
        {
            fn drop(&mut self) {}
        }
    }

}
