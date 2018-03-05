use std::{fs, path};

use config::Config;
use store::Store;

fn reset(name: &str) -> String {
    let s = format!("./test/{}", name);
    let _ = fs::remove_dir_all(&s);
    s
}

#[test]
fn test_basic() {
    let path = reset("basic");

    // Create a new store
    let cfg = Config::default(path.clone());
    let store = Store::<&str>::new(cfg).unwrap();
    let bucket = store.default().unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let mut txn = store.write_txn::<&str>().unwrap();
    txn.set(bucket, "testing", "abc123").unwrap();
    txn.commit().unwrap();

    let txn = store.read_txn::<&str>().unwrap();
    assert_eq!(txn.get(bucket, "testing").unwrap(), "abc123");
    txn.abort();
}

#[test]
fn test_integer_keys() {
    let path = reset("integer_keys");

    // Create a new store
    let cfg = Config::default(path.clone());
    let store = Store::new_integer_keys(cfg).unwrap();
    let bucket = store.default().unwrap();
    assert!(path::Path::new(path.as_str()).exists());

    let mut txn = store.write_txn::<&str>().unwrap();
    let key = 0x1234;
    txn.set(bucket, key.into(), "abc123").unwrap();
    txn.commit().unwrap();

    let txn = store.read_txn::<&str>().unwrap();
    assert_eq!(txn.get(bucket, key.into()).unwrap(), "abc123");
    txn.abort();
}
