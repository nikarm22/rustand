use rustand::{Store, global_store, store_actions};

#[global_store]
#[derive(Default, Clone)]
struct State {
    count: u32,
}

#[store_actions]
impl Store<State> {
    fn get_count(&self) -> u32 {
        self.get().count
    }

    fn set_count(&self, new_val: u32) {
        self.set(|s| s.count = new_val);
    }
}

fn main() {
    // Retrieve the global store instance.
    let store = State::store();

    println!("Initial count: {}", store.get_count());

    let _sub = store.subscribe(|state| {
        println!("Subscriber notified. New count: {}", state.count);
    });

    {
        let store2 = State::store();

        let _sub2 = store2.subscribe(|state| {
            println!("Subscriber 2 notified. New count: {}", state.count);
        });

        store.set_count(100);
    }

    store.set_count(1);
    store.set_count(2);

    println!("Final count: {}", store.get().count);
}
