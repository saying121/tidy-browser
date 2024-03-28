#![allow(clippy::string_slice)]

use secret_service::{EncryptionType, SecretService};

#[ignore = "just inspect"]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[cfg(target_os = "linux")]
async fn all_pass() {
    // initialize secret service (dbus connection and encryption session)
    let ss = SecretService::connect(EncryptionType::Dh)
        .await
        .unwrap();
    // get default collection
    let collection = ss
        // .get_all_collections()
        .get_default_collection()
        .await
        .unwrap();
    if collection
        .is_locked()
        .await
        .unwrap()
    {
        collection.unlock().await.unwrap();
    }
    let coll = collection
        .get_all_items()
        .await
        .unwrap();
    for i in coll {
        let lab = i.get_label().await.unwrap();
        dbg!(lab);
        let res = i.get_secret().await.unwrap();
        let pass = String::from_utf8_lossy(&res).to_string();
        println!(r##"(| pass |) -> {}"##, &pass[..50.min(pass.len())]);
    }
}