/*
Type parameter defaults can only use parameters that occur before them.
*/

#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]

    fn with_error1() {
        //        struct Foo<T=U, U=()> {
        //            field1: T,
        //            filed2: U,
        //        }
        //// error: type parameters with a default cannot use forward declared
        //// identifiers
    }
    #[allow(dead_code)]
    #[test]
    //cargo test --test e0128 without_error1 -- --nocapture
    fn without_error1() {
        #[derive(Debug)]
        struct Foo<U = (), T = U> {
            field1: T,
            filed2: U,
        }
        let value = Foo {
            field1: true,
            filed2: true,
        };
        println!("{:?}", value)
    }

    #[allow(dead_code)]
    #[test]
    //cargo test --test e0128 without_error2 -- --nocapture
    fn without_error2() {
        #[derive(Debug)]
        struct Foo<U = (), T = U> {
            field1: T,
            filed2: U,
        }
        let value = Foo {
            field1: (),
            filed2: (),
        };
        println!("{:?}", value)
    }
}
