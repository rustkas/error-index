/*
Types in type definitions have lifetimes associated with them that represent how long
the data stored within them is guaranteed to be live.
*/
// cargo test --test e0309 with_error1
// cargo test --test e0309 without_error1
#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        //no error
        // This won't compile because T is not constrained, meaning the data
        // stored in it is not guaranteed to last as long as the reference
        struct Foo<'a, T> {
            foo: &'a T,
        }
    }
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {
        struct Foo<'a, T> {
            foo: &'a T,
        }
        let v = "42".to_string();
        let _f = Foo { foo: &v };
        // drop(v);
        // println!("{}", f.foo); // but we've already dropped v!
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        struct Foo<'a, T: 'a> {
            foo: &'a T,
        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {}
}
