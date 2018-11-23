#![feature(box_syntax)]
/*
A value was moved. However, its size was not known at compile time, and only values of a known
size can be moved.
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(unused_imports)]
    #[test]
    // In Rust, you can only move a value when its size is known at compile time.
    fn with_error() {
        //        let array: &[isize] = &[1, 2, 3];
        //        let _x: Box<[isize]> = box *array;
        //        // error: cannot move a value of type [isize]: the size of [isize] cannot
        //        //        be statically determined
    }

    #[allow(dead_code)]
    #[allow(unused_imports)]
    #[test]
    // To work around this restriction, consider "hiding" the value behind a reference: either &x or
    // &mut x. Since a reference has a fixed size, this lets you move it around as usual.
    fn without_error1() {
        let array: &[isize] = &[1, 2, 3];
        let _x: Box<&[isize]> = box array; // ok!
    }

}
