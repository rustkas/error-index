/*
An associated function for a trait was defined to be a method (i.e. to take a self parameter),
but an implementation of the trait declared the same function to be static.
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]
    fn with_error1() {
        //        trait Foo {
        //            fn foo(&self);
        //        }
        //
        //        struct Bar;
        //
        //        impl Foo for Bar {
        //            // error, method `foo` has a `&self` declaration in the trait, but not in
        //            // the impl
        //            fn foo() {}
        //        }
    }

    #[allow(dead_code)]
    #[test]
    fn without_error1() {
        trait Foo {
            fn foo();
        }

        struct Bar;

        impl Foo for Bar {
            // error, method `foo` has a `&self` declaration in the impl, but not in
            // the trait
            fn foo() {}
        }
    }

    #[allow(dead_code)]
    #[test]
    fn without_error2() {
        trait Foo {
            fn foo(&self);
        }

        struct Bar;

        impl Foo for Bar {
            // error, method `foo` has a `&self` declaration in the impl, but not in
            // the trait
            fn foo(&self) {}
        }
    }
}
