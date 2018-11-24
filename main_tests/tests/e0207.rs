/*
Disallow unconstrained type parameters from impls.
Any type parameter or lifetime parameter of an impl must meet at least one of the following criteria:

* it appears in the self type of the impl
* for a trait impl, it appears in the trait reference
* it is bound as an associated type

*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
        //        struct Foo;
        //
        //        impl<T: Default> Foo {
        //            // error: the type parameter `T` is not constrained by the impl trait, self
        //            // type, or predicates [E0207]
        //            fn get(&self) -> T {
        //                <T as Default>::default()
        //            }
        //        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error2() {
        //This fails to compile because T does not appear in the trait or in the implementing type.

        //        trait Maker {
        //            type Item;
        //            fn make(&mut self) -> Self::Item;
        //        }
        //
        //        struct Foo<T> {
        //            foo: T,
        //        }
        //
        //        struct FooMaker;
        //
        //        impl<T: Default> Maker for FooMaker {
        //            // error: the type parameter `T` is not constrained by the impl trait, self
        //            // type, or predicates [E0207]
        //            type Item = Foo<T>;
        //
        //            fn make(&mut self) -> Foo<T> {
        //                Foo {
        //                    foo: <T as Default>::default(),
        //                }
        //            }
        //        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        //The problem is that the parameter T does not appear in the self type (Foo) of the impl.
        // In this case, we can fix the error by moving the type parameter from the impl
        // to the method get:
        struct Foo;

        // Move the type parameter from the impl to the method
        impl Foo {
            fn get<T: Default>(&self) -> T {
                <T as Default>::default()
            }
        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error2() {
        // One way to work around this is to introduce a phantom type parameter into FooMaker,
        // like so:

        use std::marker::PhantomData;

        trait Maker {
            type Item;
            fn make(&mut self) -> Self::Item;
        }

        struct Foo<T> {
            foo: T,
        }

        // Add a type parameter to `FooMaker`
        struct FooMaker<T> {
            phantom: PhantomData<T>,
        }

        impl<T: Default> Maker for FooMaker<T> {
            type Item = Foo<T>;

            fn make(&mut self) -> Foo<T> {
                Foo {
                    foo: <T as Default>::default(),
                }
            }
        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    // Another way is to do away with the associated type in Maker and use
    // an input type parameter instead:
    fn without_error3() {
        // One way to work around this is to introduce a phantom type parameter into FooMaker,
        // like so:

        // Use a type parameter instead of an associated type here
        trait Maker<Item> {
            fn make(&mut self) -> Item;
        }

        struct Foo<T> {
            foo: T,
        }

        struct FooMaker;

        impl<T: Default> Maker<Foo<T>> for FooMaker {
            fn make(&mut self) -> Foo<T> {
                Foo {
                    foo: <T as Default>::default(),
                }
            }
        }
    }
    // For more information, please see RFC 447.
    // https://github.com/rust-lang/rfcs/blob/master/text/0447-no-unused-impl-parameters.md
}
