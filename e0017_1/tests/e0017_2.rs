#[macro_use]
extern crate lazy_static;

/*

cargo test --test e0017_2 without_error1  -- --nocapture


*/

#[cfg(test)]
mod tests {

    use std::sync::Mutex;

    lazy_static! {
     static mut ref C1: Mutex<i32> = Mutex::new(2);
    }

    #[test]
    fn without_error1() {
        let mut CR = C1.lock().unwrap();

        println!("{}", CR);
        //assert_eq!(3, CR);
    }
}
