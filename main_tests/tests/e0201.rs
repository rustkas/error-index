/*
It is an error to define two associated items (like methods, associated types, associated functions,
etc.) with the same identifier.
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
        //        struct Foo(u8);
        //
        //        impl Foo {
        //            fn bar(&self) -> bool { self.0 > 5 }
        //            fn bar() {} // error: duplicate associated function
        //        }
        //
        //        trait Baz {
        //            type Quux;
        //            fn baz(&self) -> bool;
        //        }
        //
        //        impl Baz for Foo {
        //            type Quux = u32;
        //
        //            fn baz(&self) -> bool { true }
        //
        //            // error: duplicate method
        //            fn baz(&self) -> bool { self.0 > 5 }
        //
        //            // error: duplicate associated type
        //            type Quux = u32;
        //        }

    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        struct Foo(u8);

        impl Foo {
            fn bar(&self) -> bool {
                self.0 > 5
            }
        }

        trait Baz {
            type Quux;
            fn baz(&self) -> bool;
        }

        impl Baz for Foo {
            type Quux = u32;

            fn baz(&self) -> bool {
                true
            }
        }
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error2() {
        struct Foo<T>(T);

        impl Foo<u8> {
            fn bar(&self) -> bool {
                self.0 > 5
            }
        }

        impl Foo<bool> {
            fn bar(&self) -> bool {
                self.0
            }
        }
    }
}
