/*
cargo test --test e0038
cargo test --test e0038 with_error -- --nocapture
cargo test --test e0038 with_error1 -- --nocapture
cargo test --test e0038 without_error1 -- --nocapture
cargo test --test e0038 without_error2 -- --nocapture

*/

/*
Trait objects like Box<Trait> can only be constructed when certain requirements are satisfied by
the trait in question.

Trait objects are a form of dynamic dispatch and use a dynamically sized type for the inner type.
So, for a given trait Trait, when Trait is treated as a type, as in Box<Trait>, the inner type is
'unsized'. In such cases the boxed pointer is a 'fat pointer' that contains an extra pointer
to a table of methods (among other things) for dynamic dispatch. This design mandates some
restrictions on the types of traits that are allowed to be used in trait objects, which are
collectively termed as 'object safety' rules.

Rulls:

-- The trait cannot require Self: Sized
-- Method references the Self type in its arguments or return type


*/

#[cfg(test)]

mod tests {
    use std::fmt::Debug;

    #[test]
    //  cargo test --test e0038 with_error1 -- --nocapture
    // -- The trait cannot require Self: Sized
    fn with_error1() {
        {
            trait Foo1
            where
                Self: Sized,
                Self: Debug,
            {
            }

            //TODO uncomment the code below to produce the error E0038

            //            #[derive(Debug)]
            //            #[warn(dead_code)]
            //            struct FooImpl;
            //            impl Foo1 for FooImpl {}
            //
            //
            //    let obj:Box<Foo1> = Box::new(FooImpl);
            //                println!("{:?}",obj);
        }

        {
            // The same rul without error
            trait Foo1
            where
                // just comment the line below
                //                    Self: Sized,
                Self: Debug,
            {
            }

            #[derive(Debug)]
            struct FooImpl;
            impl Foo1 for FooImpl {}

            //TODO uncomment the code below to produce the error E0038
            let obj: Box<Foo1> = Box::new(FooImpl);
            println!("{:?}", obj);
        }
    }

    #[test]
    // -- Method references the Self type in its arguments or return type
    // cargo test --test e0038 with_error2 -- --nocapture
    fn with_error2() {
        trait Trait {
            //Now, foo() can no longer be called on a trait object, but you will now be allowed
            // to make a trait object, and that will be able to call any object-safe methods.
            // With such a bound, one can still call foo() on types implementing that trait that
            // aren't behind trait objects.
            fn foo1(&self) -> Self
            where
                Self: Sized;
            fn foo2(&mut self) -> Self;
            //               fn foo2(&mut self) -> Self where Self: Sized;
        }

        impl Trait for String {
            fn foo1(&self) -> Self {
                "hi".to_owned()
            }
            fn foo2(&mut self) -> Self {
                "hi".to_owned()
            }
        }

        impl Trait for u8 {
            fn foo1(&self) -> Self {
                1
            }
            fn foo2(&mut self) -> Self {
                1
            }
        }
        //TODO uncomment the code below to produce the error E0038
        //            fn call_foo1(x: Box<Trait>) {
        //                let y = x.foo1(); // What type is y?
        //                y
        //                // ...
        //            }
        //
        //            fn call_foo2(mut x: Box<Trait>) {
        //                let y = x.foo2(); // What type is y?
        //                y
        //                // ...
        //            }
        //
        //            impl Trait for Box<Trait>{
        //                fn foo1(&self) -> Self {
        //                    Box::new(&self)
        //                }
        //                fn foo2(&mut self) -> Self {
        //                    Box::new(&mut self)
        //                }
        //            }
    }

    #[test]
    // cargo test --test e0038 with_error3 -- --nocapture
    // Method has generic type parameters
    fn with_error3() {
        {
            trait Trait {
                fn foo(&self);
            }

            impl Trait for String {
                fn foo(&self) {
                    // implementation 1
                }
            }

            impl Trait for u8 {
                fn foo(&self) {
                    // implementation 2
                }
            }
        }
        // At compile time each implementation of Trait will produce a table containing the various
        // methods (and other items) related to the implementation.

        // TODO uncomment the code below
        //        {
        //            fn foo<T>(x: T) {
        //                // ...
        //            }
        //
        //            println!("{}", foo::<u8>(1u8));
        //            println!("{}", foo::<bool>(true));
        //            println!("{}", foo::<String>("".to_string()));
        //        }

        {
            trait Trait {
                fn foo<T>(&self, _on: T);
                // more methods
            }

            impl Trait for String {
                fn foo<T>(&self, _on: T) {
                    // implementation 1
                }
            }

            impl Trait for u8 {
                fn foo<T>(&self, _on: T) {
                    // implementation 2
                }
            }
        }

        {
            trait Trait {
                fn foo<T>(&self, _on: T)
                where
                    Self: Sized;
                // more methods
            }

            impl Trait for String {
                fn foo<T>(&self, _on: T) {
                    // implementation 1
                }
            }

            impl Trait for u8 {
                fn foo<T>(&self, _on: T) {
                    // implementation 2
                }
            }

            //TODO uncomment the code below to produce an error E0038

            //            fn call_foo(thing: Box<Trait>) {
            ////                thing.foo(true); // this could be any one of the 8 types above
            ////                thing.foo(1);
            ////                thing.foo("hello");
            //                thing.foo("".to_string());
            //            }
            //            let string_value = "".to_string();
            //            call_foo(Box::new(string_value));
        }
    }

}

#[test]
// --Method has no receiver
pub fn with_error4() {
    trait Foo {
        fn foo() -> u8
        where
            Self: Sized;
    }
}

#[test]
// -- The trait cannot contain associated constants
pub fn with_error5() {

    // TODO: uncomment the code below to produce the error
    //    trait Foo {
    //        const X: i32;
    //    }
    //
    //    impl Foo {}
}

#[test]
// -- The trait cannot use Self as a type parameter in the supertrait listing
fn with_error6() {
    {
        trait Super<A> {
            fn get_a(&self) -> A; // note that this is object safe!
        }
        // TODO: uncomment the code below to produce the error
        //        trait Trait: Super<Self> {
        //        }
        trait Trait {}
        struct Foo;

        impl Super<Foo> for Foo {
            fn get_a(&self) -> Foo {
                Foo {}
            }
        }

        impl Trait for Foo {}
    }
}
