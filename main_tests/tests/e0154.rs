/*
Imports (use statements) are not allowed after non-item statements, such as variable declarations
and expression statements.
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(unused_imports)]
    #[test]
    // no error
    fn with_error() {
        fn f() {
            // Variable declaration before import
            let x = 0;
            use std::io::Read;
            println!("{}", x);
        }
    }

    #[allow(dead_code)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        fn f() {
            use std::io::Read;
            let x = 0;
            println!("{}", x);
        }
    }

}
