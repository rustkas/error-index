/*
An attempt was made to implement Drop on a trait, which is not allowed: only structs and enums
can implement Drop.
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]
    //cargo test --test e0120 with_error1 -- --nocapture
    fn with_error1() {
        //        trait MyTrait {}
        //
        //        impl Drop for MyTrait {
        //            fn drop(&mut self) {}
        //        }
    }
    #[allow(dead_code)]
    #[test]
    //cargo test --test e0120 without_error1 -- --nocapture
    // A workaround for this problem is to wrap the trait up in a struct, and implement Drop on that.
    fn without_error1() {
        trait MyTrait {
            fn my_trait_drop(&self) {
                println!("MyTrait drop");
            }
        }
        struct MyWrapper<T: MyTrait> {
            foo: T,
        }

        impl<T: MyTrait> Drop for MyWrapper<T> {
            fn drop(&mut self) {
                self.foo.my_trait_drop();
            }
        }
        struct MyTraitImpl;
        impl MyTrait for MyTraitImpl {}

        let _value = MyWrapper {
            foo: MyTraitImpl {},
        };
    }
    #[allow(dead_code)]
    #[test]
    //cargo test --test e0120 without_error2 -- --nocapture
    //  wrapping trait objects requires something like the following:
    fn without_error2() {
        trait MyTrait {
            fn my_trait_drop(&self) {
                println!("MyTrait drop");
            }
        }

        //or Box<MyTrait>, if you wanted an owned trait object
        struct MyWrapper<'a> {
            foo: &'a MyTrait,
        }

        impl<'a> Drop for MyWrapper<'a> {
            fn drop(&mut self) {
                self.foo.my_trait_drop();
            }
        }
        struct MyTraitImpl;
        impl MyTrait for MyTraitImpl {}

        let _value = MyWrapper {
            foo: &MyTraitImpl {},
        };
    }
    #[allow(dead_code)]
    #[test]
    //cargo test --test e0120 without_error3 -- --nocapture
    //  wrapping trait objects requires something like the following:
    fn without_error3() {
        trait MyTrait {
            fn my_trait_drop(&self) {
                println!("MyTrait drop");
            }
        }

        //or Box<MyTrait>, if you wanted an owned trait object
        struct MyWrapper {
            foo: Box<MyTrait>,
        }

        impl Drop for MyWrapper {
            fn drop(&mut self) {
                self.foo.my_trait_drop();
            }
        }
        struct MyTraitImpl;
        impl MyTrait for MyTraitImpl {}

        let _value = MyWrapper {
            foo: Box::new(MyTraitImpl {}),
        };
    }
}
