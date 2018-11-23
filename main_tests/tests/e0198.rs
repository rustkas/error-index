#![feature(optin_builtin_traits)]

/*
A negative implementation is one that excludes a type from implementing a particular trait.
Not being able to use a trait is always a safe operation, so negative implementations are always
safe and never need to be marked as unsafe.
*/

/*
An auto trait is the new name for the terribly named opt-in, built-in trait (OIBIT).

These are an unstable feature where a trait is automatically implemented for every type unless
they opt-out or contain a value that does not implement the trait:
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
//        struct Foo;
//
//        // unsafe is unnecessary
//        unsafe impl !Clone for Foo { }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        struct Foo;

        auto trait Enterprise {}

        impl !Enterprise for Foo { }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error2() {

        auto trait IsCool {}

        // Everyone knows that `String`s just aren't cool
        impl !IsCool for String {}

        struct MyStruct;
        struct HasAString(String);

        fn check_cool<C: IsCool>(_: C) {}


            check_cool(42);
            check_cool(false);
            check_cool(MyStruct);

            // the trait bound `std::string::String: IsCool` is not satisfied
            // check_cool(String::new());

            // the trait bound `std::string::String: IsCool` is not satisfied in `HasAString`
            // check_cool(HasAString(String::new()));

    }
}
