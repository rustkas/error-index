/*
This error occurs when there was a recursive trait requirement that overflowed before
it could be evaluated. Often this means that there is unbounded recursion in resolving some type bounds.


*/

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        //        trait Foo {}
        //
        //        struct Bar<T>(T);
        //
        //        impl<T> Foo for T where Bar<T>: Foo {}
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        trait Foo {}
        trait Foo1 {}
        struct Bar<T>(T);

        impl<T> Foo for T where Bar<T>: Foo1 {}
    }

}
