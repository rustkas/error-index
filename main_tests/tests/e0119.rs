/*


*/

/*
There are conflicting trait implementations for the same type.
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]
    //cargo test --test e0119 with_error1 -- --nocapture
    fn with_error1() {
        trait MyTrait {
            fn get(&self) -> usize;
        }

        impl<T> MyTrait for T {
            fn get(&self) -> usize {
                0
            }
        }

        struct Foo {
            value: usize,
        }

        let value = Foo { value: 90 };
        let result = value.get();
        println!("{}", result);
        //
        //        impl MyTrait for Foo { // error: conflicting implementations of trait
        //        //        `MyTrait` for type `Foo`
        //        fn get(&self) -> usize { self.value }
        //        }
    }
    #[allow(dead_code)]
    #[test]
    //cargo test --test e01198 without_error1 -- --nocapture
    fn without_error1() {
        trait MyTrait {
            fn get(&self) -> usize;
        }

        impl<T> MyTrait for T {
            fn get(&self) -> usize {
                0
            }
        }

        struct Foo {
            value: usize,
        }

        let value = Foo { value: 9 };
        let result = value.get();
        println!("{}", result);
    }
    #[allow(dead_code)]
    #[test]
    //cargo test --test e0118 without_error2 -- --nocapture
    fn without_error2() {}

}
