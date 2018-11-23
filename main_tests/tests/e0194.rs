/*
A type parameter was declared which shadows an existing one.
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
        //        trait Foo<T> {
        //            fn do_something(&self) -> T;
        //            fn do_something_else<T: Clone>(&self, bar: T);
        //        }

        /*
        In this example, the trait Foo and the trait method do_something_else both define a type
        parameter T.
        This is not allowed: if the method wishes to define a type parameter, it must use
        a different name for it.

        */
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        trait Foo<T> {
            fn do_something(&self) -> T;
            fn do_something_else<K: Clone>(&self, bar: K);
        }
    }
}
