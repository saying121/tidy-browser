#[cfg(target_os = "linux")]
pub mod linux;
// #[cfg(target_os = "macos")]
pub mod macos;
// #[cfg(target_os = "windows")]
pub mod win;

pub mod crypto;
pub mod path;

const S_CHROMIUM: [&str; 2] = ["Chromium", "Chromium Safe Storage"];
const S_CHROME: [&str; 2] = ["Chrome", "Chrome Safe Storage"];
const S_EDGE: [&str; 2] = ["Microsoft Edge", "Microsoft Edge Safe Storage"];
const S_BRAVE: [&str; 2] = ["Brave", "Brave Safe Storage"];
const S_YANDEX: [&str; 2] = ["Yandex", "Yandex Safe Storage"];
const S_VIVALDI: [&str; 2] = ["Vivaldi", "Vivaldi Safe Storage"];
const S_OPERA: [&str; 2] = ["Opera", "Opera Safe Storage"];
const S_OPERA_GX: [&str; 2] = ["Opera", "Opera Safe Storage"];
