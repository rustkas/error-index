/*
const and static mean different things. A const is a compile-time constant, an alias for a literal value.
This property means you can match it directly within a pattern.

The static keyword, on the other hand, guarantees a fixed location in memory.
This does not always mean that the value is constant. For example, a global mutex can be declared
static as well.
*/
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(unused_imports)]
    #[test]
    // no error
    fn with_error() {}

    #[allow(dead_code)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        static FORTY_TWO: i32 = 42;

        match Some(42) {
            Some(x) if x == FORTY_TWO => {}
            _ => {}
        }
    }

}
