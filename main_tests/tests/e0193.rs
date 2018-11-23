/*
where clauses must use generic type parameters: it does not make sense to use them otherwise.
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn with_error1() {
        trait Foo {
            fn bar(&self);
        }

        #[derive(Copy, Clone)]
        struct Wrapper<T> {
            Wrapped: T,
        }

        impl Foo for Wrapper<u32>
        where
            Wrapper<u32>: Clone,
        {
            fn bar(&self) {}
        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[test]
    fn without_error1() {
        trait Foo {
            fn bar(&self);
        }

        #[derive(Copy, Clone)]
        struct Wrapper<T> {
            Wrapped: T,
        }
        impl<T> Foo for Wrapper<T>
        where
            Wrapper<T>: Clone,
        {
            fn bar(&self) {}
        }
    }
}
