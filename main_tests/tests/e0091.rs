#![feature(repr_simd)]
/*
cargo test --test e0075
cargo test --test e0075 with_error -- --nocapture
cargo test --test e0075 without_error1 -- --nocapture

*/

/*
You gave an unnecessary type parameter in a type alias
*/

#[cfg(test)]
mod tests {
    #[test]
    fn with_error1() {
        {
          //  type Foo<T> = u32; // error: type parameter `T` is unused
        }

        // or:
        {
        //    type Foo<A, B> = Box<A>; // error: type parameter `B` is unused
        }
    }

    #[test]
    fn without_error1() {
        {
            #[allow(dead_code)]
            type Foo = u32; // ok!
        }
        {
            #[allow(dead_code)]
            type Foo2<A> = Box<A>; // ok!
        }
    }
}
