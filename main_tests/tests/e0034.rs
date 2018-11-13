// #![allow(unused)]

/*
cargo test --test e0034
cargo test --test e0034 with_error -- --nocapture
cargo test --test e0034 without_error1 -- --nocapture

*/

/*
The compiler doesn't know what method to call because more than one method has the same prototype.
*/

#[cfg(test)]
mod tests {

    struct Test;

    #[test]
    fn with_error() {
        trait Trait1 {
            fn foo();
        }

        trait Trait2 {
            fn foo();
        }
        trait Trait3 {
            fn foo();
        }

        trait Trait4 {
            fn foo();
        }

        impl Trait1 for Test {
            fn foo() {
                println!("trait 1")
            }
        }
        impl Trait2 for Test {
            fn foo() {
                println!("trait 2")
            }
        }
        impl Trait3 for Test {
            fn foo() {
                println!("trait 3")
            }
        }
        impl Trait4 for Test {
            fn foo() {
                println!("trait 4")
            }
        }

        //TODO uncomment the lines below
        // Test::foo() // error, which foo() to call?
    }

    #[test]
    fn without_error1() {
        trait Trait5 {
            fn empty();
        }
        impl Trait5 for Test {
            fn empty() {
                println!("trait 5")
            }
        }
        Test::empty();
    }
    #[test]
    fn without_error2() {
        trait Trait1 {
            fn foo();
        }

        trait Trait2 {
            fn foo();
        }
        impl Trait1 for Test {
            fn foo() {
                println!("trait 1")
            }
        }
        impl Trait2 for Test {
            fn foo() {
                println!("trait 2")
            }
        }

        <Test as Trait1>::foo();
        <Test as Trait2>::foo();
    }
    #[test]
    fn without_error3() {
        trait F {
            fn m(&self);
        }

        trait G {
            fn m(&self);
        }

        struct X;

        impl F for X {
            fn m(&self) {
                println!("I am F");
            }
        }
        impl G for X {
            fn m(&self) {
                println!("I am G");
            }
        }

        let f = X;

        F::m(&f); // it displays "I am F"
        G::m(&f); // it displays "I am G"
    }
}
