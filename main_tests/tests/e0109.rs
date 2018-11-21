/*
You tried to give a type parameter to a type which doesn't need it.
*/

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[test]
    fn with_error1() {
        // type X = u32<i32>; // error: type parameters are not allowed on this type
        type X = u32;

        //let _value_wrong = Option::<u32>::None;
        let _value = Option::None::<u32>;
    }

    #[allow(dead_code)]
    #[test]
    fn without_error1() {
        type X = u32;

        let _value = Option::None::<u32>;
    }

}
