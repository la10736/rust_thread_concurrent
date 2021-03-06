use std::thread::spawn;

fn main() {
    static N: usize = 10000;
    let r_val = std::sync::Arc::new(std::sync::Mutex::new(0));
    let mut handlers: Vec<_> = Default::default();

    for _ in 0..N {
        let r_val = r_val.clone();
        handlers.push(spawn(move || {
            let mut r_val = r_val.lock().unwrap();
            *r_val += 1;
        }
        ));
    }
    for h in handlers {
        h.join().unwrap();
    }
    let result = r_val.lock().unwrap().clone();
    print!("Sum = {}", result);
    assert_eq!(N, result);
}