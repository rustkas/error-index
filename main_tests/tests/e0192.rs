/*
Negative impls are only allowed for auto traits. For more information see the opt-in builtin traits RFC.
https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md
*/

// I do not understand this conception
#[cfg(test)]
mod tests {
    //    use std::rc::Rc;

    #[allow(dead_code)]
    #[test]
    fn with_error1() {
        //        trait Snapshot { }
        //        impl Snapshot for .. { }
        //
        //        // In general, anything that can reach interior mutability is not
        //// snapshotable.
        //        impl<T> !Snapshot for Unsafe<T> { }
        //
        //        // But it's ok for Rc<T>.
        //        impl<T:Snapshot> Snapshot for Rc<T> { }
    }

    #[allow(dead_code)]
    #[test]
    fn without_error1() {}
}
