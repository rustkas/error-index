/*
You declared a pattern as an argument in a foreign function declaration.
*/
/*
repr_transparent
This feature enables the repr(transparent) attribute on structs, which enables the use of newtypes
without the usual ABI implications of wrapping the value in a struct.
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]

    fn with_error1() {
        extern "C" {
            //            fn foo((a, b): (u32, u32)); // error: patterns aren't allowed in foreign
            //            //        function declarations
        }
    }
    #[allow(dead_code)]
    #[test]

    fn without_error1() {
        //        use std::marker::PhantomData;
        //        #[repr(transparent)]
        //        struct SomeStruct<'a, T: 'a> {
        //            a: u32,
        //            b: u32,
        //            phantom: PhantomData<&'a T>,
        //        }
        //
        //        extern {
        //            fn foo(s: SomeStruct<bool>); // ok!
        //        }
    }
    #[allow(dead_code)]
    #[test]

    fn without_error2() {
        use std::marker::PhantomData;

        struct Slice<'a, T: 'a> {
            start: *const T,
            end: *const T,
            phantom: PhantomData<&'a T>,
        }
    }
    #[allow(dead_code)]
    #[test]
    fn without_error3() {
        use std::ops::Add;

        struct Millimeters(f64);
        struct Grams(f64);

        impl Add<Millimeters> for Millimeters {
            type Output = Millimeters;

            fn add(self, other: Millimeters) -> Millimeters {
                Millimeters(self.0 + other.0)
            }
        }
    }

}
