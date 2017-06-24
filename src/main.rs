use std::thread::spawn;

fn main() {
    static N: usize = 10000;
    let r_val = std::sync::Arc::new(0);
    let mut handlers: Vec<_> = Default::default();

    for _ in 0..N {
        let r_val = r_val.clone();
        handlers.push(spawn(move || {
            *r_val += 1;
        }
        ));
    }
    for h in handlers {
        h.join().unwrap();
    }
    print!("Sum = {}", r_val);
    assert_eq!(N, *r_val);
}