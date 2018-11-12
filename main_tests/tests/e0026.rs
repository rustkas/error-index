// #![allow(unused)]

/*
cargo test --test e0026
cargo test --test e0026 with_error -- --nocapture
cargo test --test e0026 without_error1 -- --nocapture

*/

/*
This error indicates that a struct pattern attempted to extract a non-existent field from a struct.
*/

#[cfg(test)]
mod tests {

    struct Thing {
        x: u32,
        y: u32,
    }

    #[test]
    fn with_error() {
        let thing = Thing { x: 0, y: 0 };

        match thing {
            //TODO uncomment the line below
            // Thing { x, z } => {println!("{} {}", x,z);}
            Thing { x, y } => {
                println!("{} {}", x, y);
            }
        }
    }

    #[test]
    fn without_error1() {
        let thing = Thing { x: 0, y: 0 };

        match thing {
            Thing { x, y } => {
                println!("{} {}", x, y);
            } // _ => {}
        }
    }

}
