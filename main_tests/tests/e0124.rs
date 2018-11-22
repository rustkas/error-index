/*

*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]

    fn with_error1() {
        //        struct Foo {
        //            field1: i32,
        //            field1: i32, // error: field is already declared
        //        }
    }
    #[allow(dead_code)]
    #[test]
    //cargo test --test e0124 without_error1 -- --nocapture
    fn without_error1() {
        #[derive(Debug)]
        struct Foo {
            field1: i32,
            field2: i32, // error: field is already declared
        }
        let value = Foo {
            field1: 9,
            field2: 8,
        };
        println!("{:?}", value)
    }

}
