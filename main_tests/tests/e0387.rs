/*
This error occurs when an attempt is made to mutate or mutably reference data that a closure has
captured immutably.
*/

// cargo test --test e0365 with_error1 -- --nocapture
// cargo test --test e0387 without_error1 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        //        // Accepts a function or a closure that captures its environment immutably.
        //        // Closures passed to foo will not be able to mutate their closed-over state.
        //        fn foo<F: Fn()>(_f: F) {}
        //
        //        // Attempts to mutate closed-over data. Error message reads:
        //        // `cannot assign to data in a captured outer variable...`
        //        fn mutable() {
        //            let mut x = 0u32;
        //            foo(|| x = 2);
        //        }
        //
        //        // Attempts to take a mutable reference to closed-over data.  Error message
        //        // reads: `cannot borrow data mutably in a captured outer variable...`
        //        fn mut_addr() {
        //            let mut x = 0u32;
        //            foo(|| {
        //                let _y = &mut x;
        //            });
        //        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        fn foo<F: FnMut()>(_f: F) {}

        // Attempts to mutate closed-over data. Error message reads:
        // `cannot assign to data in a captured outer variable...`
        fn mutable() {
            let mut x = 0u32;
            foo(|| x = 2);
        }

        // Attempts to take a mutable reference to closed-over data.  Error message
        // reads: `cannot borrow data mutably in a captured outer variable...`
        fn mut_addr() {
            let mut x = 0u32;
            foo(|| {
                let _y = &mut x;
            });
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        use std::cell::Cell;
        use std::cell::RefCell;
        fn foo<F: Fn()>(_f: F) {}

        fn mutable() {
            let x = Cell::new(0u32);
            foo(|| x.set(2));
        }

        fn mut_addr() {
            let x = RefCell::new(0u32);
            foo(|| {
                let mut _y = x.borrow_mut();
                let mut _y = &x.borrow_mut();
            });
        }
    }
}
