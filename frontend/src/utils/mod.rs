use web_sys::window;

pub use device_info::DeviceInfo;
pub use get_title_bar::get_titlebar;

pub mod filetree;

pub fn alert<T: std::fmt::Debug>(message: &T) {
    let window = window().unwrap();
    window
        .alert_with_message(&format!("{:?}", message))
        .unwrap();
}

mod get_title_bar;
mod device_info;
