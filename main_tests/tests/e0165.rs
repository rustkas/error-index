/*
A while-let pattern attempts to match the pattern, and enters the body if the match was successful.
If the match is irrefutable (when it cannot fail to match), use a regular let-binding inside
a loop instead.
*/
#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        //        struct Irrefutable(i32);
        //        let irr = Irrefutable(0);
        //
        //// This fails to compile because the match is irrefutable.
        //        while let Irrefutable(x) = irr {
        //            println!("{}", x);
        //        }
    }

    #[test]
    fn without_error1() {
        struct Irrefutable(i32);
        let irr = Irrefutable(0);

        loop {
            let Irrefutable(x) = irr;
            println!("{}", x);
            break;
        }
    }
}
