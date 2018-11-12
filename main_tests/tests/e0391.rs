/*
cargo test --test e0391
cargo test --test e0391 with_error
cargo test --test e0391 without_error1

*/

/*
This error indicates that some types or traits depend on each other and therefore cannot be constructed.
*/

#[cfg(test)]
mod tests {

    #[test]
    fn with_error() {
        // TODO uncomment the line below
        //        trait FirstTrait : SecondTrait {
        //
        //        }

        // TODO comment the line below
        trait FirstTrait {}
        trait SecondTrait: FirstTrait {}
    }

    #[test]
    fn without_error1() {
        trait FirstTrait {}
        trait SecondTrait {}
        trait ThirdTrait: SecondTrait {}
    }

}
