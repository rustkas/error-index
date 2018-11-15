/*
cargo test --test e0070
cargo test --test e0070 with_error -- --nocapture
cargo test --test e0070 without_error1 -- --nocapture

*/

/*
The left-hand side of an assignment operator must be a place expression.
*/
// I lot of problems with recursion_limit
#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        //        struct SomeStruct {
        //            x: i32,
        //            y: i32
        //        }
        //
        //        const SOME_CONST : i32 = 12;
        //
        //        fn some_other_func() {}
        //
        //        fn some_function() {
        //            SOME_CONST = 14; // error : a constant value cannot be changed!
        //            1 = 3; // error : 1 isn't a valid place!
        //            some_other_func() = 4; // error : we can't assign value to a function!
        //            SomeStruct.x = 12; // error : SomeStruct a structure name but it is used
        //            // like a variable!
        //        }
    }
    #[test]
    fn without_error1() {
        struct SomeStruct {
            x: i32,
            y: i32,
        }
        let mut s = SomeStruct { x: 0, y: 0 };

        s.x = 3; // that's good !
        s.y = 3; // that's good too!

        // ...

        fn some_func(x: &mut i32) {
            *x = 12; // that's good !
        }
        let mut x = 12;
        some_func(&mut x);
    }
}
