/*
An attempt was made to mutate data using a non-mutable reference. This commonly occurs when attempting
to assign to a non-mutable reference of a mutable reference (&(&mut T)).
*/

// cargo test --test e0389 with_error1 -- --nocapture
// cargo test --test e0389 without_error1 -- --nocapture
#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    pub fn with_error1() {
        //        struct FancyNum {
        //            num: u8,
        //        }
        //
        //        let mut fancy = FancyNum { num: 5 };
        //        let fancy_ref = &(&mut fancy);
        //        fancy_ref.num = 6; // error: cannot assign to data in a `&` reference
        //        println!("{}", fancy_ref.num);
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error2() {}

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error1() {
        struct FancyNum {
            num: u8,
        }

        let mut fancy = FancyNum { num: 5 };

        let fancy_ref = &mut fancy;
        // `fancy_ref` is now &mut FancyNum, rather than &(&mut FancyNum)

        fancy_ref.num = 6; // No error!

        println!("{}", fancy_ref.num);
    }

    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn without_error2() {
        struct FancyNum {
            num: u8,
        }

        let mut fancy = FancyNum { num: 5 };

        let fancy_ref = &mut (&mut fancy);
        // `fancy_ref` is now &mut(&mut FancyNum), rather than &(&mut FancyNum)

        fancy_ref.num = 6; // No error!

        println!("{}", fancy_ref.num);
    }
}
