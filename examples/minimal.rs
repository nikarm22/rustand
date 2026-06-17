use rustand::Store;

#[derive(Default, Clone)]
struct State {
    count: u32,
}

fn main() {
    let store = Store::new(State::default());

    let _sub = store
        .subscribe(|new_value| {
            println!("Subscriber notified. New count: {}", new_value.count);
        });

    store.set(|state| state.count += 1);
    store.set(|state| state.count += 1);
    store.set(|state| state.count -= 1);

    println!("Final count: {}", store.get().count);
}
