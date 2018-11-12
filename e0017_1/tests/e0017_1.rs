#[macro_use]
extern crate lazy_static;

/*

cargo test --test e0017_1 without_error1  -- --nocapture


*/

#[cfg(test)]
mod tests {

    use std::sync::Mutex;

    lazy_static! {
        static ref TABLE: Mutex<[KV; 100]> = Mutex::new([KV { k: -1, v: -1 }; 100]);
    }

    #[derive(Debug, Clone, Copy)]
    struct KV {
        k: i32,
        v: i32,
    }

    fn update_table(i: usize, elem: KV) {
        let mut table = TABLE.lock().unwrap();
        table[i] = elem;
    }

    fn read_table(i: usize) -> KV {
        let table = TABLE.lock().unwrap();
        table[i]
    }

    #[test]
    fn without_error1() {
        update_table(5, KV { k: 23, v: 35 });

        let line: KV = read_table(5);
        println!("{} {}", line.k, line.v);
        assert_eq!(23, line.k);
        assert_eq!(35, line.v);
    }

}
