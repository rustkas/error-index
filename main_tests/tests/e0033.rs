// #![allow(unused)]

/*
cargo test --test e0033
cargo test --test e0033 with_error -- --nocapture
cargo test --test e0033 without_error1 -- --nocapture

*/

/*
This error indicates that a pointer to a trait type cannot be implicitly dereferenced by a pattern.
Every trait defines a type, but because the size of trait implementors isn't fixed, this type has no
compile-time size. Therefore, all accesses to trait types must be through pointers.
If you encounter this error you should try to avoid dereferencing the pointer.
https://doc.rust-lang.org/reference/types.html#trait-objects
*/

#[cfg(test)]
mod tests {

    trait SomeTrait {
        fn method_one(&self);
        fn method_two(&self);
    }
    impl SomeTrait for &'static str {
        fn method_one(&self) {
            println!("{}", "one");
        }
        fn method_two(&self) {
            println!("{}", "two");
        }
    }

    #[test]
    fn with_error() {
        let _trait_obj: &SomeTrait = &"some_value";

        // This tries to implicitly dereference to create an unsized local variable.
        //TODO uncomment the lines below
        //        let &_invalid = _trait_obj;
        //                invalid.method_one();
        //                invalid.method_two();
    }

    #[test]
    fn without_error1() {
        let trait_obj: &SomeTrait = &"some_value";
        // You can call methods without binding to the value being pointed at.
        trait_obj.method_one();
        trait_obj.method_two();
    }

}
