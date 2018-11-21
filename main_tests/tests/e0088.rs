/*
cargo test --test e0075
cargo test --test e0075 with_error -- --nocapture
cargo test --test e0075 without_error1 -- --nocapture

*/

/*
You gave too many lifetime arguments
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        fn f() {}

        //        fn main() {
        //            f::<'static>() // error: wrong number of lifetime arguments:
        //            //        expected 0, found 1
        //        }
        f();
    }
    #[test]
    fn without_error1() {
        struct Foo {
            value: String,
        }

        impl Foo {
            // it can be written like this
            fn get_value<'a>(&'a self) -> &'a str {
                &self.value
            }
            // but the compiler works fine with this too:
            fn without_lifetime(&self) -> &str {
                &self.value
            }
        }

        let f = Foo {
            value: "hello".to_owned(),
        };

        println!("{}", f.get_value());
        println!("{}", f.without_lifetime());
    }
}
