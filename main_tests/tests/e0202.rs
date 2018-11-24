/*
Inherent associated types were part of RFC 195 but are not yet implemented. See the tracking issue
for the status of this implementation.
https://github.com/rust-lang/rust/issues/8995
https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {

        //        trait Add<Rhs> {
        //            type Sum; // Sum is an *output*
        //            fn add(&self, &Rhs) -> Sum;
        //        }
        //
        //        impl Add<int> for int {
        //            type Sum = int;
        //            fn add(&self, rhs: &int) -> int { 0 }
        //        }
        //
        //        impl Add<Complex> for int {
        //            type Sum = Complex;
        //            fn add(&self, rhs: &Complex) -> Complex { 0 }
        //        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        trait Graph<N, E> {
            fn has_edge(&self, &N, &N) -> bool;
        }
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error2() {}
}
