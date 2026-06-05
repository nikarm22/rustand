use rustand::Store;

#[tokio::main]
async fn main() {
    let store = Store::new(0);

    let _sub = store
        .subscribe(|v| println!("Count is now: {}", v))
        .await
        .unwrap();

    println!("Incrementing...");
    store.set(|s| *s += 1).await.unwrap();

    println!("Incrementing again...");
    store.set(|s| *s += 1).await.unwrap();

    println!("Final value: {}", store.get().await.unwrap());
}
