/*
This error occurs when an attempt is made to use data captured by a closure, when that data
may no longer exist.
*/

// cargo test --test e0365 with_error1 -- --nocapture
// cargo test --test e0365 without_error1 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        //        fn foo() -> Box<Fn(u32) -> u32> {
        //            let x = 0u32;
        //            Box::new(|y| x + y)
        //        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {
        //        fn foo() {
        //            let x = 0u32;
        //            let y = 1u32;
        //
        //            let thr = std::thread::spawn(|| {
        //                x + y
        //            });
        //        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        fn foo() -> Box<Fn(u32) -> u32> {
            let x = 0u32;
            Box::new(move |y| x + y)
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        fn foo() {
            let x = 0u32;
            let y = 1u32;

            let _thr = std::thread::spawn(move || x + y);
        }
    }
}
