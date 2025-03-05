use core::fmt;
use std::{ffi::OsString, time::Duration};

use futures_core::Stream;

pub trait PlatformAPI {
    fn get_activity_stream(rate: Duration) -> crate::Result<impl Stream<Item = ActivityInfo>>;
    fn get_all_window_info() -> crate::Result<Vec<WindowInfo>>;
}

#[derive(Default, Debug, Clone)]
pub struct ActivityInfo {
    pub window: Option<WindowInfo>,
    pub idle_time: u32,
}

#[derive(Clone)]
pub struct WindowInfo {
    pub pid: u32,
    pub path: OsString,
}

impl fmt::Debug for WindowInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{} {}", self.pid, self.path.to_string_lossy()))
    }
}
