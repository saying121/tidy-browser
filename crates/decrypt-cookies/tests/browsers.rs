use decrypt_cookies::Browser;

#[test]
fn browsers() {
    let a = Browser::chromiums();
    for ele in a {
        assert_ne!(ele, Browser::Librewolf);
        assert_ne!(ele, Browser::Firefox);
        #[cfg(target_os = "macos")]
        assert_ne!(ele, Browser::Safari);
    }
    let a = Browser::firefoxs();
    for ele in a {
        match ele {
            Browser::Librewolf | Browser::Firefox => {},
            _ => panic!(""),
        }
    }
}
