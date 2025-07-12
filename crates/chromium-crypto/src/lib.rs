cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub use linux::Decrypter;
    } else if #[cfg(target_os = "macos")] {
        pub mod mac;
        pub use mac::Decrypter;
    } else if #[cfg(target_os = "windows")] {
        pub mod win;
        pub use win::Decrypter;
    }
}

pub mod error;
