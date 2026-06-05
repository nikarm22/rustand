use rustand::Store;

#[async_std::test]
async fn test_async_std_get_sync_inside_runtime() {
    let store = Store::new(20);
    assert_eq!(store.get_sync().unwrap(), 20);
}

#[test]
fn test_async_std_get_sync_outside_runtime() {
    let store = Store::new(10);
    assert_eq!(store.get_sync().unwrap(), 10);
}

#[test]
fn test_async_std_set_sync_outside_runtime() {
    let store = Store::new(0);
    store.set_sync(|s| *s = 100).unwrap();
    assert_eq!(store.get_sync().unwrap(), 100);
}
