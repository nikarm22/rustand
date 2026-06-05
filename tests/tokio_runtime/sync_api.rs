use rustand::Store;

#[test]
fn test_tokio_get_sync_outside_runtime() {
    let store = Store::new(10);
    assert_eq!(store.get_sync().unwrap(), 10);
}

#[tokio::test]
async fn test_tokio_get_sync_inside_runtime() {
    let store = Store::new(20);
    assert_eq!(store.get_sync().unwrap(), 20);
}

#[test]
fn test_tokio_set_sync_outside_runtime() {
    let store = Store::new(0);
    store.set_sync(|s| *s = 100).unwrap();
    assert_eq!(store.get_sync().unwrap(), 100);
}
