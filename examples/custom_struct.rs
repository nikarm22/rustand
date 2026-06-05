use rustand::Store;

/// A custom struct representing an application state.
#[derive(Debug, Clone)]
struct AppState {
    user_name: String,
    score: u32,
    items: Vec<String>,
}

#[tokio::main]
async fn main() {
    // Initialize the store with a custom struct
    let initial_state = AppState {
        user_name: "RustDeveloper".to_string(),
        score: 0,
        items: vec![],
    };
    let store = Store::new(initial_state);

    // Subscribe to specific changes (manual selection logic)
    let _sub = store
        .subscribe(|state| {
            println!("Subscriber: Current items: {:?}", state.items);
        })
        .await
        .unwrap();

    println!("Initial state: {:?}", store.get().await.unwrap());

    // Update specific fields within the struct
    println!("Adding items...");
    store
        .set(|state| {
            state.score += 10;
            state.items.push("Ferris".to_string());
        })
        .await
        .unwrap();

    store
        .set(|state| {
            state.user_name = "MasterRust".to_string();
            state.items.push("Crab".to_string());
        })
        .await
        .unwrap();

    let final_state = store.get().await.unwrap();
    println!("Final State: {:#?}", final_state);

    assert_eq!(final_state.score, 10);
    assert_eq!(final_state.items.len(), 2);
}
