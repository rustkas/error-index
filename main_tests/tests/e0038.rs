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

    When Trait is treated as a type, the type does not implement the special Sized trait, because
    the type does not have a known size at compile time and can only be accessed behind a pointer.
    Thus, if we have a trait like the following:

    ```
    trait Foo where Self: Sized {

    }
    ```

    We cannot create an object of type Box<Foo> or &Foo since in this case Self would not be Sized.

    Generally, Self:Sized is used to indicate that the trait should not be used as a trait object.
    If the trait comes from your own crate, consider removing this restriction.

*/

#[cfg(test)]
mod tests {



    #[test]
    fn with_error() {
        trait Foo1 {

        }
        trait Foo2 where Self: Sized {

        }
//        let obj:Foo1 = Box<Foo1>::new(a);
 //       let obj:Box<Foo1>;
        //let obj1 = Box<Foo1>::new(Foo1);
  //      println!("{:?}",obj1);
//        let obj = Foo1;
//        let obj1 = Box<Foo1>();
    }
#[test]
    fn with_error1(){
        //TODO uncomment the conde below to produce the error E0038
//        trait Trait {
//            fn foo(&self) -> Self;
//        }
//
//        fn call_foo(x: Box<Trait>) {
//            let y = x.foo(); // What type is y?
//            // ...
//        }
    }


    #[test]
    fn with_error2() {
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
    fn without_error1() {
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
}
