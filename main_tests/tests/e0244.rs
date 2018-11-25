/*
This error indicates that too many type parameters were found in a type or trait.

*/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
        //        struct Foo { x: bool }
        //
        //        struct Bar<S, T> { x: Foo<S, T> }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]

    fn without_error1() {
        struct Foo {
            x: bool,
        }

        struct Bar<S, T> {
            x: Foo,
            phantom1: std::marker::PhantomData<S>,
            phantom2: std::marker::PhantomData<T>,
        }
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    // move the type bindings in the where clause
    fn without_error2() {}
}
