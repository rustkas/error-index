/*
In types, the + type operator has low precedence, so it is often necessary to use parentheses.
*/
#[cfg(test)]
mod tests {

    #[test]
    fn with_error1() {
        //        trait Foo {}
        //
        //        struct Bar<'a> {
        //            w: &'a Foo + Copy,   // error, use &'a (Foo + Copy)
        //        }

    }
    #[test]
    fn with_error2() {
        //                trait Foo {}
        //
        //                struct Bar<'a> {
        //                    x: &'a Foo + 'a,     // error, use &'a (Foo + 'a)
        //                }
    }

    #[test]
    fn with_error3() {
        //                trait Foo {}
        //
        //                struct Bar<'a> {
        //                    y: &'a mut Foo + 'a, // error, use &'a mut (Foo + 'a)
        //                }
    }

    #[test]
    fn with_error4() {
        //        trait Foo {}
        //
        //        struct Bar<'a> {
        //            z: fn() -> Foo + 'a, // error, use fn() -> (Foo + 'a)
        //        }
    }

    #[allow(dead_code)]
    #[test]
    fn without_error1() {
        trait Foo {}

        struct Bar<'a> {
            w: &'a (Foo + Send),
        }

        struct Bar1<'a> {
            w: &'a (Foo + Sync),
        }
        struct Bar2<'a> {
            w: &'a (Foo + Send + Sync),
        }
    }

    #[allow(dead_code)]
    #[test]
    fn without_error2() {
        trait Foo {}

        struct Bar<'a> {
            x: &'a (Foo + 'a),
        }
    }

    #[allow(dead_code)]
    #[test]
    fn without_error3() {
        trait Foo {}

        struct Bar<'a> {
            y: &'a mut (Foo + 'a), // error, use &'a mut (Foo + 'a)
        }
    }
    #[allow(dead_code)]
    #[test]
    fn without_error4() {
        trait Foo {}

        struct Bar<'a> {
            z: fn() -> (Foo + 'a), // error, use fn() -> (Foo + 'a)
        }
    }
}
