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
    //    #[test]
    //    fn with_error2() {
    //        //TODO uncomment the code below to produce the error E0038
    //
    //        trait Foo where Self: Sized {
    //
    //        }

    //                trait Trait {
    //                    fn foo(&self) -> Self;
    //                }
    //
    //                fn call_foo(x: Box<Trait>) {
    //                    let y = x.foo(); // What type is y?
    //                    // ...
    //                }
}

#[test]
fn with_error3() {
    //        trait Trait {
    //            fn foo(&self) -> Self where Self: Sized;
    //            // more functions
    //        }
    //
    //        fn call_foo(x: Box<Trait>) {
    //            let y = x.foo(); // What type is y?
    //            // ...
    //        }
}

#[test]
fn without_error_1() {
    trait Trait {
        fn foo(&self) -> Self;
    }

    impl Trait for String {
        fn foo(&self) -> Self {
            "hi".to_owned()
        }
    }

    impl Trait for u8 {
        fn foo(&self) -> Self {
            1
        }
    }
}

#[test]
fn without_error3() {
    //        trait Trait {
    //            fn foo<T>(&self, on: T) where Self: Sized;
    //            // more methods
    //        }
    //
    //        impl Trait for String {
    //            fn foo<T>(&self, on: T) {
    //                // implementation 1
    //            }
    //        }
    //
    //        impl Trait for u8 {
    //            fn foo<T>(&self, on: T) {
    //                // implementation 2
    //            }
    //        }
    //
    //        fn call_foo(thing: Box<Trait>) {
    //            thing.foo(true); // this could be any one of the 8 types above
    //            thing.foo(1);
    //            thing.foo("hello");
    //        }
}
#[test]
fn without_error4() {
    trait Foo {
        fn foo() -> u8
        where
            Self: Sized;
    }
}
