/*
This error means that an attempt was made to match a struct type enum variant as a non-struct type
*/
#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        //        enum Foo { B { i: u32 } }
        //
        //        fn bar(foo: Foo) -> u32 {
        //            match foo {
        //                Foo::B(i) => i, // error E0164
        //            }
        //        }
        //        bar(Foo::B {i:9});
    }

    #[test]
    fn without_error1() {
        enum Foo {
            B { i: u32 },
        }

        fn bar(foo: Foo) -> u32 {
            match foo {
                Foo::B { i } => i,
            }
        }
        bar(Foo::B { i: 9 });
    }
}
