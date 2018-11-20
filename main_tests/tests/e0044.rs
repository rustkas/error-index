// #![allow(unused)]

/*
cargo test --test e0044
cargo test --test e0044 with_error -- --nocapture
cargo test --test e0044 without_error1 -- --nocapture

*/

/*
You can't use type parameters on foreign items.
*/

#[cfg(test)]
mod tests {

    //TODO provide examples for this error
    // extern { fn some_func<T>(x: T); }
}
