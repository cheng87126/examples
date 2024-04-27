// #[cfg_attr(target_os = "linux", path = "linux.rs")]
// #[cfg_attr(target_os = "macos", path = "macos.rs")]
// #[cfg_attr(windows, path = "windows.rs")]
// mod os;

mod open_url;

use crate::open_url::open;

fn main() {
    open();
    // os::open();
}