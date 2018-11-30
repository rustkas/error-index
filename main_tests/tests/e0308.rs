/*
This error occurs when the compiler was unable to infer the concrete type of a variable.
It can occur for several cases, the most common of which is a mismatch in the expected type
that the compiler inferred for a variable's initializing expression, and the actual type
explicitly assigned to the variable.
*/

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        //let x: i32 = "I am not a number!";
        //     ~~~   ~~~~~~~~~~~~~~~~~~~~
        //      |             |
        //      |    initializing expression;
        //      |    compiler infers type `&str`
        //      |
        //    type `i32` assigned to variable `x`

    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {
        let _x = "I am not a number!";
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {}

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
