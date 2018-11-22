#![feature(start)]
/*
A function with the start attribute was declared with type parameters.
It is not possible to declare type parameters on a function that has the start attribute.

#[start] controlling the entry point is possible in two ways:
the #[start] attribute, or overriding the default shim for the C main function with your own.
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]

    fn with_error1() {
        #[start]
        fn f<T>() {}
    }

    #[allow(dead_code)]
    #[test]

    fn without_error1() {
        #[start]
        fn my_start(_argc: isize, _argv: *const *const u8) -> isize {
            0
        }
    }
}
