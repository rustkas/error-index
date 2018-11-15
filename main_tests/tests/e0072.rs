/*
cargo test --test e0072
cargo test --test e0072 with_error -- --nocapture
cargo test --test e0072 without_error1 -- --nocapture

*/

/*
When defining a recursive struct or enum, any use of the type being defined from inside the definition
must occur behind a pointer (like Box or &). This is because structs and enums must have
a well-defined size, and without the pointer, the size of the type would need to be unbounded.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        //        // error, invalid recursive struct type
        //        struct ListNode {
        //            head: u8,
        //            tail: Option<ListNode>,
        //        }
    }
    #[test]
    fn without_error1() {
        #[allow(dead_code)]
        struct ListNode {
            head: u8,
            tail: Option<Box<ListNode>>,
        }
    }
}
