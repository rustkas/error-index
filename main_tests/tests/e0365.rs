#![feature(core_intrinsics)]

pub fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}
/*
Private modules cannot be publicly re-exported. This error indicates that you attempted to pub use
a module that was not itself public.
*/

// cargo test --test e0365 with_error1 -- --nocapture
// cargo test --test e0365 without_error1 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    mod foo {

        pub const X: u32 = 1;
    }

    pub mod foo2 {
        pub const X: u32 = 1;
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        mod foo_inside {
            pub const X: u32 = 1;
            pub fn myfn() {}
        }

        //super::print_type_of(&foo_inside::myfn);
        // pub use self::with_error1::foo_inside as foo2;
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        mod foo {
            pub const X: u32 = 1;
        }

        pub use tests::foo2 as foo_2;
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
