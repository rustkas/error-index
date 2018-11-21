/*
This error means that an incorrect number of generic arguments were provided
*/

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[test]
    fn with_error1() {
        struct Foo<T> {
            x: T,
        }

        //        struct Bar { x: Foo }             // error: wrong number of type arguments:
        //        // expected 1, found 0
        //
        struct Bar {
            x: Foo<bool>,
        }

        //        struct Baz<S, T> { x: Foo<S, T> } // error: wrong number of type arguments:
        //        //        expected 1, found 2
        struct Baz<S, T> {
            x: Foo<S>,
            y: Foo<T>,
        }

        fn foo<T, U>(_x: T, _y: U) {}

        let x: bool = true;
        //        foo::<bool>(x);                 // error: wrong number of type arguments:
        //        //        expected 2, found 1
        //        foo::<bool, i32, i32>(x, 2, 4); // error: wrong number of type arguments:
        //        //        expected 2, found 3
        foo::<bool, bool>(x, x);
    }

    #[allow(dead_code)]
    #[test]
    fn without_error1() {
        struct Foo<T> {
            x: T,
        }

        struct Bar {
            x: Foo<bool>,
        }

        struct Baz<S, T> {
            x: Foo<S>,
            y: Foo<T>,
        }

        fn foo<T, U>(_x: T, _y: U) {}

        let x: bool = true;
        foo::<bool, bool>(x, x);
    }

}
