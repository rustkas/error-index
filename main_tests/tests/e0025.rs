// #![allow(unused)]

/*
cargo test --test e0025
cargo test --test e0025 with_error
cargo test --test e0025 without_error1 -- --nocapture

*/

/*
Each field of a struct can only be bound once in a pattern.
*/

#[cfg(test)]
mod tests {

    struct Foo {
        a: u8,
        b: u8,
    }

    #[test]
    fn with_error() {
        let x = Foo { a: 1, b: 2 };

        // let Foo { a: x, a: y } = x;
        // error: field `a` bound multiple times in the patter
        let Foo { a: _x, b: _y } = x;
    }

    #[test]
    fn without_error1() {
        let x = Foo { a: 1, b: 2 };

        // let Foo { a: x, a: y } = x;
        // error: field `a` bound multiple times in the patter
        let Foo { a: x, b: y } = x;
        println!("{} {}", x, y);
    }

}
