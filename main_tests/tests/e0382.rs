/*
This error occurs when an attempt is made to use a variable after its contents have been moved elsewhere.
*/

// cargo test --test e0365 with_error1 -- --nocapture
// cargo test --test e0382 without_error2 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {

//        struct MyStruct { s: u32 }
//
//            let mut x = MyStruct{ s: 5u32 };
//            let y = x;
//            x.s = 6;
//            println!("{}", x.s);

    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {

            let s1 = String::from("hello");

            let len = calculate_length(&s1);

            println!("The length of '{}' is {}.", s1, len);


        fn calculate_length(s: &String) -> usize {
            s.len()
        }

    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        use std::cell::RefCell;
        use std::rc::Rc;

        struct MyStruct { s: u32 }


            let x = Rc::new(RefCell::new(MyStruct{ s: 5u32 }));
            let y = x.clone();
            x.borrow_mut().s = 6;
            println!("{}", x.borrow().s);

    }
}
