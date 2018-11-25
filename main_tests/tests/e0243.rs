/*
This error indicates that not enough type parameters were found in a type or trait.

*/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
        //        struct Foo<T> { x: T }
        //
        //        struct Bar { x: Foo }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    // move the type bindings in the type parameter declaration
    fn without_error1() {
        struct Foo<T> {
            x: T,
        }

        struct Bar {
            x: Foo<bool>,
        }
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    // move the type bindings in the where clause
    fn without_error2() {}
}
