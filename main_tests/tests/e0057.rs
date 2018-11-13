//#![recursion_limit="6"]

/*
cargo test --test e0057
cargo test --test e0057 with_error -- --nocapture
cargo test --test e0057 without_error1 -- --nocapture

*/

/*
When invoking closures or other implementations of the function traits Fn, FnMut or FnOnce using
call notation, the number of parameters passed to the function must match its definition.
*/
// I lot of problems with recursion_limit
#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        let f: fn(_) -> _ = |x| x * 3;

        //  let a = f();        // invalid, too few parameters
        let _b = f(4); // this works!
                       // let c = f(2, 3);    // invalid, too many parameters

        fn foo<F: Fn()>(f: F) {
            f(); // this is valid, but f(3) would not work
        }

        let k = || ();
        foo(k);
    }
    #[test]
    fn without_error1() {
        let f: fn(_) -> _ = |x| x * 3;
        let _b = f(4); // this works!

        fn foo<F: Fn()>(f: F) {
            f(); // this is valid, but f(3) would not work
        }

        let k = || ();
        foo(k);
    }
}
