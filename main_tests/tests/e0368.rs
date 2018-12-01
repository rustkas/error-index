/*
This error indicates that a binary assignment operator like += or ^= was applied to a type that
doesn't support it.
*/

// cargo test --test e0365 with_error1 -- --nocapture
// cargo test --test e0365 without_error1 -- --nocapture
// cargo test --test e0368 without_error2 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        //        let mut x = 12f32; // error: binary operation `<<` cannot be applied to
        //        //        type `f32`
        //
        //        x <<= 2;
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        let mut _x = 12u32; // the `u32` type does implement the `ShlAssign` trait

        _x <<= 2; // ok!
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        use std::ops::Add;
        use std::ops::AddAssign;
        #[derive(Debug)]
        struct Foo(u32);

        impl Add for Foo {
            type Output = Foo;

            fn add(self, rhs: Foo) -> Foo {
                Foo(self.0 + rhs.0)
            }
        }
        impl AddAssign for Foo {
            fn add_assign(&mut self, other: Foo) {
                self.0 = self.0 + other.0;
            }
        }

        let mut x: Foo = Foo(5);
        x += Foo(7);
        println!("{:?}", x);
    }

}
