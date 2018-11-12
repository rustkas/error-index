/*
cargo test --test e0017
cargo test --test e0017 with_error
cargo test --test e0017 with_error1
cargo test --test e0017 with_error2

*/

/*
References in statics and constants may only refer to immutable values.
Statics are shared everywhere, and if they refer to mutable data one might violate memory safety
since holding multiple mutable references to shared data is not allowed.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        static _X: i32 = 1;

        // these three are not allowed:

        //TODO uncomment the line below
        //static STATIC_REF: &'static mut i32 = &mut X;
    }

    #[test]
    fn with_error1() {
        const CONSTANT: i32 = 2;

        //TODO uncomment some below
        // these three are not allowed:
        // const CR: &'static mut i32 = &mut C;
        //static CONST_REF: &'static mut i32 = &mut C;
        // static CONST_REF:
        // &'static mut i32 = &mut C;
        const _C1: i32 = CONSTANT;
        const _C2: i32 = CONSTANT;
        const _C3: i32 = CONSTANT;
        println!("{}, {}, {}", _C1, _C2, _C3)
    }

    #[test]
    fn without_error1() {
        const CONSTANT: i32 = 2;
        const _C1: i32 = CONSTANT;
        const _C2: i32 = CONSTANT;
        const _C3: i32 = CONSTANT;
        println!("{}, {}, {}", _C1, _C2, _C3)
    }

}
