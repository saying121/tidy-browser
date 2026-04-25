cfg_select! {
    target_os = "linux" => {
        pub mod linux;
        pub use linux::Decrypter;
    }
    target_os = "macos" => {
        pub mod mac;
        pub use mac::Decrypter;
    }
    target_os = "windows" => {
        pub mod win;
        pub use win::Decrypter;
    }
    _ => {
        compile_error!("Not support the platform");
    }
}

pub mod error;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Which {
    Cookie,
    Login,
}
