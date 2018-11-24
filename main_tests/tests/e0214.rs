/*
A generic type was described using parentheses rather than angle brackets.

*/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {

    //let _v: Vec(&str) = vec!["foo"];


    }


    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        let _v: Vec<&str> = vec!["foo"];
    }



}
