/*
cargo test --test e0106
cargo test --test e0106 with_error -- --nocapture
cargo test --test e0106 without_error1 -- --nocapture

*/

/*
This error indicates that a lifetime is missing from a type.
*/

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[test]
    fn with_error1() {
        // struct Foo1 { x: &bool }

        struct Foo2;

        struct Foo1<'a> {
            x: &'a bool,
        }

        struct Bar1 {
            x: Foo2,
        }

        //struct Bar2<'a> { x: Foo2<'a> } // incorrect
        struct Bar2<'a> {
            x: &'a Foo2,
        } // correct
        struct Bar3 {
            x: Foo2,
        } // correct

        // enum Baz1 { A(u8), B(&bool), } // incorrect
        enum Baz1<'a> {
            A(u8),
            B(&'a bool),
        } // correct
        enum Baz2<'a> {
            A(u8),
            B(&'a bool),
        } // correct

        // type MyStr1 = &str; // incorrect
        type MyStr2<'a> = &'a str; // correct
    }

    #[allow(dead_code)]
    #[test]
    fn with_error2() {
        // error, no input lifetimes
        // fn foo() -> &str { }

        fn foo<'a>() -> &'a str {
            ""
        }

        // error, `x` and `y` have distinct lifetimes inferred
        //fn bar(x: &str, y: &str) -> &str { }

        fn bar<'a>(_x: &'a str, _y: &str) -> &'a str {
            ""
        }

        // error, `y`'s lifetime is inferred to be distinct from `x`'s
        fn baz<'a>(_x: &'a str, _y: &str) -> &'a str {
            ""
        }
    }

    #[allow(dead_code)]
    #[test]
    fn without_error1() {
        struct Foo2;

        struct Foo1<'a> {
            x: &'a bool,
        }

        struct Bar1 {
            x: Foo2,
        }

        struct Bar2<'a> {
            x: &'a Foo2,
        } // correct
        struct Bar3 {
            x: Foo2,
        } // correct

        enum Baz1<'a> {
            A(u8),
            B(&'a bool),
        } // correct
        enum Baz2<'a> {
            A(u8),
            B(&'a bool),
        } // correct

        type MyStr2<'a> = &'a str; // correct
    }

    #[allow(dead_code)]
    #[test]
    fn without_error2() {
        fn foo<'a>() -> &'a str {
            ""
        }

        fn bar<'a>(_x: &'a str, _y: &str) -> &'a str {
            ""
        }

        fn baz<'a>(_x: &'a str, _y: &str) -> &'a str {
            ""
        }
    }
}
