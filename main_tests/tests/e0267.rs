
/*
This error indicates the use of a loop keyword (break or continue) inside
a closure but outside of any loop
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
      //  let _w = || { break; }; // error: `break` inside of a closure

    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]

    fn without_error1() {
        let w = || {
            for _ in 0..10 {
                break;
            }
        };

        w();

    }

}
