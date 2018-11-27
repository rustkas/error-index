#![feature(lang_items)]
/*
An unknown external lang item was used
*/
/**/
extern crate foo;
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]
    fn with_error1() {

        //        extern "C" {
        //            #[lang = "cake"] // error: unknown external lang item: `cake`
        //            fn cake();
        //        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    #[test]

    fn without_error1() {}

}
