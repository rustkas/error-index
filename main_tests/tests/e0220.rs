/*
You used an associated type which isn't defined in the trait.

*/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {

        //        trait T1 {
        //            type Bar;
        //        }
        //
        //        type Foo = T1<F=i32>; // error: associated type `F` not found for `T1`
        //
        //// or:
        //
        //        trait T2 {
        //            type Bar;
        //
        //            // error: Baz is used but not declared
        //            fn return_bool(&self, _: &Self::Bar, _: &Self::Baz) -> bool;
        //        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        //Make sure that you have defined the associated type in the trait body.
        // Also, verify that you used the right trait or you didn't misspell
        // the associated type name. Example:

        trait T1 {
            type Bar;
        }

        type Foo = T1<Bar = i32>; // ok!

        // or:

        trait T2 {
            type Bar;
            type Baz; // we declare `Baz` in our trait.

            // and now we can use it here:
            fn return_bool(&self, _: &Self::Bar, _: &Self::Baz) -> bool;
        }
    }

}
