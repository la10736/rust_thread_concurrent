use std::thread::spawn;

fn main() {
    static N: usize = 10000;
    let mut val = 0;
    let r_val = &mut val;
    let mut handlers: Vec<_> = Default::default();

    for _ in 0..N {
        handlers.push(spawn(|| {
            *r_val += 1;
        }
        ));
    }
    for h in handlers {
        h.join().unwrap();
    }
    print!("Sum = {}", val);
    assert_eq!(N, val);
}