/*
cargo test --test e0015
cargo test --test e0015 with_error  -- --nocapture
cargo test --test e0015 without_error1  -- --nocapture

*/

/*
The only functions that can be called in static or constant expressions are const functions, and
struct/enum constructors. const functions are only available on a nightly compiler. Rust currently
does not support more general compile-time function execution.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        const _FOO0: u8 = 0;
        //TODO uncomment the line below
        //        fn function()->u8{2+2+_FOO0}
        //        static _FOO3:u8 = function();
    }

    #[test]
    fn without_error1() {
        const _FOO: Option<u8> = Some(1); // enum constructor
        const _FOO0: u8 = 0;
        #[allow(dead_code)]
        struct Bar {
            x: u8,
        }
        const _BAR: Bar = Bar { x: 1 }; // struct constructor
        const fn const_function() -> u8 {
            2 + 2 + _FOO0
        }

        const _FOO1: u8 = const_function();
        static _FOO2: u8 = const_function();
        println!("{}", _FOO1);
        println!("{}", _FOO2);

        let value: u8 = const_function();
        println!("{}", value);
    }

}
