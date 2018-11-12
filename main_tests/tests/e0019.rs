// #![allow(unused)]

/*
cargo test --test e0019
cargo test --test e0019 with_error
cargo test --test e0019 without_error1
cargo test --test e0019 without_error2

*/

/*
A function call isn't allowed in the const's initialization expression because the expression's
value must be known at compile-time.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        enum Test {
            V1,
        }
        #[allow(dead_code)]
        impl Test {
            fn test(&self) -> i32 {
                12
            }
        }

        const FOO: Test = Test::V1;
        //TODO uncomment below and get an erro E0015 (?)
        //  const A: i32 = FOO.test(); // You can't call Test::func() here!
        drop(FOO);
    }

    #[test]
    fn without_error1() {
        enum Test {
            V1,
        }

        impl Test {
            // (!) add const here. It works
            const fn test(&self) -> i32 {
                12
            }
        }

        const FOO: Test = Test::V1;
        const _A1: i32 = FOO.test();
        const _A2: i32 = FOO.test();
        drop(_A1);
        drop(_A2);
        drop(FOO);
    }

    #[test]
    fn without_error2() {
        enum Test {
            V1,
        }

        impl Test {
            fn func(&self) -> i32 {
                12
            }
        }

        const FOO: Test = Test::V1;

        FOO.func(); // here is good
        let _x = FOO.func(); // or even here!
    }

}
