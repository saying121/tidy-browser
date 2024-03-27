#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn safari_binary_cookies() {
use decrypt_cookies::SafariBuilder;
    let var  = SafariBuilder::new().build().unwrap();
}
