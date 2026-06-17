use arbitrary::Arbitrary;
use rustand::Store;
use std::sync::{Arc, Mutex};

#[derive(Arbitrary, Debug)]
pub enum Operation {
    Get,
    Set(u32),
    Subscribe,
    Unsubscribe(usize),
}

pub fn run_fuzz(ops: Vec<Operation>) {
    let store = Store::new(0u32);
    let subs = Arc::new(Mutex::new(Vec::new()));

    for op in ops {
        match op {
            Operation::Get => {
                let _ = store.get();
            }
            Operation::Set(val) => {
                store.set(move |s| *s = val);
            }
            Operation::Subscribe => {
                let mut s = subs.lock().unwrap();
                let sub = store.subscribe(|_| {});
                s.push(sub);
            }
            Operation::Unsubscribe(idx) => {
                let mut s = subs.lock().unwrap();
                if !s.is_empty() {
                    let idx = idx % s.len();
                    s.remove(idx);
                }
            }
        }
    }
}
