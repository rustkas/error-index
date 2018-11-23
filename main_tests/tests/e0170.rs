/*
Enum variants are qualified by default.
*/
#[cfg(test)]
mod tests {

    #[test]
    fn without_error1() {
        enum Method {
            GET,
            POST,
        }
        let m = Method::GET;

        match m {
            Method::GET => println!("GET"),
            Method::POST => println!("POST"),
        }
        let m = Method::POST;
        match m {
            Method::GET => println!("GET"),
            Method::POST => println!("POST"),
        }
    }
    enum Method {
        GET,
        POST,
    }
    #[test]
    fn without_error2() {
        use crate::tests::Method::{GET, POST};
        let m = GET;

        match m {
            GET => println!("GET"),
            POST => println!("POST"),
        }
        let m = POST;
        match m {
            GET => println!("GET"),
            POST => println!("POST"),
        }
    }
}
