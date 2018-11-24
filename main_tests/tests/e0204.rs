/*
An attempt to implement the Copy trait for a struct failed because one of the fields does not
implement Copy. To fix this, you must implement Copy for the mentioned field.
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {

        //        struct Foo {
        //            foo : Vec<u32>,
        //        }
        //
        //        impl Copy for Foo { }
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error2() {

        //        #[derive(Copy)]
        //        struct Foo<'a> {
        //            ty: &'a mut bool,
        //        }
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        #[derive(Copy, Clone)]
        struct Foo<'a> {
            ty: &'a bool,
        }
    }

}
