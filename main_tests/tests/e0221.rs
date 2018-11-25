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

        //        trait T1 {}
        //        trait T2 {}
        //
        //        trait Foo {
        //            type A: T1;
        //        }
        //
        //        trait Bar : Foo {
        //            type A: T2;
        //            fn do_something() {
        //                let _: Self::A;
        //            }
        //        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        trait T1 {}
        trait T2 {}

        trait Foo {
            type A: T1;
        }

        trait Bar: Foo {
            type B: T2;
            fn do_something() {
                let _: Self::A;
            }
        }
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error2() {
        trait T1 {}
        trait T2 {}

        trait Foo {
            type A: T1;
        }

        trait Bar: Foo {
            type A: T2;
            fn do_something() {
                let _: <Self as Bar>::A;
            }
        }
    }

}
