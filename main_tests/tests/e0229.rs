/*
An associated type binding was done outside of the type parameter declaration and where clause.

*/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
        //        pub trait Foo {
        //            type A;
        //            fn boo(&self) -> <Self as Foo>::A;
        //        }
        //
        //        struct Bar;
        //
        //        impl Foo for isize {
        //            type A = usize;
        //            fn boo(&self) -> usize { 42 }
        //        }
        //
        //        fn baz<I>(x: &<I as Foo<A=Bar>>::A) {}
        //// error: associated type bindings are not allowed here
        //        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    // move the type bindings in the type parameter declaration
    fn without_error1() {
        pub trait Foo {
            type A;
            fn boo(&self) -> <Self as Foo>::A;
        }

        struct Bar;

        impl Foo for isize {
            type A = usize;
            fn boo(&self) -> usize {
                42
            }
        }
        fn baz<I: Foo<A = Bar>>(x: &<I as Foo>::A) {} // ok!
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    // move the type bindings in the where clause
    fn without_error2() {
        pub trait Foo {
            type A;
            fn boo(&self) -> <Self as Foo>::A;
        }

        struct Bar;

        impl Foo for isize {
            type A = usize;
            fn boo(&self) -> usize {
                42
            }
        }
        fn baz<I>(x: &<I as Foo>::A)
        where
            I: Foo<A = Bar>,
        {
        }
    }
}
