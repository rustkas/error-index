/*
This is because of a type mismatch between the associated type of some trait (e.g. T::Bar, where T
implements trait Quux { type Bar; }) and another type U that is required to be equal to T::Bar,
but is not.
*/

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {
        //        trait Trait { type AssociatedType; }
        //
        //        fn foo<T>(t: T) where T: Trait<AssociatedType=u32> {
        //            println!("in foo");
        //        }
        //
        //        impl Trait for i8 { type AssociatedType = &'static str; }
        //
        //        foo(3_i8);
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {
        trait Trait {
            type AssociatedType;
        }

        fn foo<T>(t: T)
        where
            T: Trait<AssociatedType = u32>,
        {
            //                    ~~~~~~~~ ~~~~~~~~~~~~~~~~~~
            //                        |            |
            //         This says `foo` can         |
            //           only be used with         |
            //              some type that         |
            //         implements `Trait`.         |
            //                                     |
            //                             This says not only must
            //                             `T` be an impl of `Trait`
            //                             but also that the impl
            //                             must assign the type `u32`
            //                             to the associated type.
            println!("in foo");
        }

        //        impl Trait for i8 {
        //            type AssociatedType = &'static str;
        //        }
        //        //~~~~~~~~~~~~~~~   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        //        //      |                             |
        //        // `i8` does have                     |
        //        // implementation                     |
        //        // of `Trait`...                      |
        //        //                     ... but it is an implementation
        //        //                     that assigns `&'static str` to
        //        //                     the associated type.
        //
        //        foo(3_i8);
        //        // Here, we invoke `foo` with an `i8`, which does not satisfy
        //        // the constraint `<i8 as Trait>::AssociatedType=u32`, and
        //        // therefore the type-checker complains with this error code.
        //    }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        // Basic Example:
        trait Trait {
            type AssociatedType;
        }

        fn foo<T>(t: T)
        where
            T: Trait<AssociatedType = &'static str>,
        {
            println!("in foo");
        }

        impl Trait for i8 {
            type AssociatedType = &'static str;
        }

        foo(3_i8);

        // For-Loop Example:
        let vs = vec![1, 2, 3, 4];
        for v in &vs {
            match v {
                &1 => {}
                _ => {}
            }
        }
    }

}
