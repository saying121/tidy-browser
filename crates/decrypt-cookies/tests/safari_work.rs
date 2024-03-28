#[cfg(target_os = "macos")]
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn safari_binary_cookies() {
    use decrypt_cookies::SafariBuilder;
    let safari = SafariBuilder::new()
        .build()
        .unwrap();
}

#[test]
fn feature() {
    let var = vec![1, 2, 4, 3];
    let temp = &var[..];
    let res: [i32; 4] = temp.into();
    dbg!(res);
}
