/*
It is not possible to define main with generic parameters. When main is present,
it must take no arguments and return ().
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[test]

    fn with_error1() {
        fn main<T>() {
            // error: main function is not allowed to have generic parameters
        }
    }

}
