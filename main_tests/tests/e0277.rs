/*
You tried to use a type which doesn't implement some trait in a place which expected that trait.
*/
// cargo test --test e0276 without_error1 -- --nocapture
#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        //        // here we declare the Foo trait with a bar method
        //        trait Foo {
        //            fn bar(&self);
        //        }
        //
        //        // we now declare a function which takes an object implementing the Foo trait
        //        fn some_func<T: Foo>(foo: T) {
        //            foo.bar();
        //        }
        //
        //
        //            // we now call the method with the i32 type, which doesn't implement
        //            // the Foo trait
        //            some_func(5i32); // error: the trait bound `i32 : Foo` is not satisfied

    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {
        //        fn some_func<T>(foo: T) {
        //            println!("{:?}", foo); // error: the trait `core::fmt::Debug` is not
        //            //        implemented for the type `T`
        //        }
        //
        //
        //            // We now call the method with the i32 type,
        //            // which *does* implement the Debug trait.
        //            some_func(5i32);

    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        trait Foo {
            fn bar(&self);
        }

        fn some_func<T: Foo>(foo: T) {
            foo.bar(); // we can now use this method since i32 implements the
                       // Foo trait
        }

        // we implement the trait on the i32 type
        impl Foo for i32 {
            fn bar(&self) {}
        }
        some_func(5i32); // ok!
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        use std::fmt;

        // Restrict the input type to types that implement Debug.
        fn some_func<T: fmt::Debug>(foo: T) {
            println!("{:?}", foo);
        }

        // Calling the method is still fine, as i32 implements Debug.
        some_func(5i32);

        // This would fail to compile now:
        // struct WithoutDebug;
        // some_func(WithoutDebug);
    }
}
