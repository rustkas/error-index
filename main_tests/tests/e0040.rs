// #![allow(unused)]

/*
cargo test --test e0040
cargo test --test e0040 with_error -- --nocapture
cargo test --test e0040 without_error1 -- --nocapture

*/

/*
It is not allowed to manually call destructors in Rust. It is also not necessary to do this since
drop is called automatically whenever a value goes out of scope.
*/

#[cfg(test)]
mod tests {
#[allow(dead_code)]
    struct Foo {
        x: i32,
    }

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("kaboom");
        }
    }

    #[test]
    fn with_error() {

            let _x = Foo { x: -7 };
        // TODO error is here
          //  _x.drop(); // error: explicit use of destructor method
        
    }

    #[test]
    fn without_error1() {
        let  _x = Foo { x: -7 };

    }
    #[test]
    fn without_error2() {
        let  x = Foo { x: -7 };
        drop(x)
    }

}
