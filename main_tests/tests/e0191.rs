/*
Trait objects need to have all associated types specified.
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]
    fn with_error1() {
        //        trait Trait {
        //            type Bar;
        //        }
        //
        //        type Foo = Trait; // error: the value of the associated type `Bar` (from
        //        //        the trait `Trait`) must be specified
    }

    #[allow(dead_code)]
    #[test]
    fn without_error1() {
        trait Trait {
            type Bar;
        }

        type Foo = Trait<Bar = i32>; // ok!
    }
}
