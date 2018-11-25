/*
You attempted to use multiple types as bounds for a closure or trait object.
Rust does not currently support this.

*/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
        //        let _: Box<dyn std::io::Read + std::io::Write>;

        //Auto traits such as Send and Sync are an exception to this rule:
        // It's possible to have bounds of one non-builtin trait, plus any number of auto traits.
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        let _: Box<dyn std::io::Read + Send + Sync>;
    }

}
