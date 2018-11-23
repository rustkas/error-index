/*
An if-let pattern attempts to match the pattern, and enters the body if the match was successful.
If the match is irrefutable (when it cannot fail to match), use a regular let-binding instead.
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(unused_imports)]
    #[test]
    fn with_error() {
        //        struct Irrefutable(i32);
        //        let irr = Irrefutable(0);
        //
        //// This fails to compile because the match is irrefutable.
        //        if let Irrefutable(x) = irr {
        //            println!("{}", x);
        //        }
    }

    #[allow(dead_code)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        struct Irrefutable(i32);
        let irr = Irrefutable(0);

        let Irrefutable(x) = irr;
        println!("{}", x);
    }
}
