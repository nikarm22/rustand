use rustand::Store;

#[tokio::main]
async fn main() {
    let store = Store::new(0);

    let _sub = store
        .subscribe(|v| println!("Count is now: {}", v))
        .unwrap();

    println!("Incrementing...");
    store.set(|s| *s += 1).unwrap();

    println!("Incrementing again...");
    store.set(|s| *s += 1).unwrap();

    println!("Final value: {}", store.get().unwrap());
}
