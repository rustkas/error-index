/*
You tried to give a lifetime parameter to a type which doesn't need it.
*/

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[test]
    fn with_error1() {
        //        type X = u32<'static>; // error: lifetime parameters are not allowed on
        //        //        this type
        type X = u32;
    }

    #[allow(dead_code)]
    #[test]
    fn without_error1() {
        type X = u32;
    }

}
