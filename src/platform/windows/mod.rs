mod error;

use super::{ActivityInfo, PlatformAPI, WindowInfo};
pub use error::*;
use futures::stream::unfold;
use futures_core::Stream;
use std::{
    collections::HashMap,
    ffi::{CStr, OsStr},
    time::Duration,
};
use tokio::time::{interval_at, Instant, MissedTickBehavior};
use windows::{
    core::PSTR,
    Win32::{
        Foundation::{HWND, MAX_PATH},
        System::{
            SystemInformation::GetTickCount,
            Threading::{
                GetProcessHandleFromHwnd, GetProcessId, QueryFullProcessImageNameA,
                PROCESS_NAME_WIN32,
            },
        },
        UI::WindowsAndMessaging::GetForegroundWindow,
    },
};
use winsafe::{EnumWindows, GetLastInputInfo};

pub struct Platform;

impl Platform {
    fn get_active_window_info() -> Option<WindowInfo> {
        unsafe {
            let window = GetForegroundWindow();
            Self::get_window_info(window)
        }
    }

    fn get_window_info(window: HWND) -> Option<WindowInfo> {
        unsafe {
            let handle = GetProcessHandleFromHwnd(window);
            let pid = GetProcessId(handle);
            let mut len = MAX_PATH;
            let mut str: Vec<u8> = vec![0; len as _];
            let res = QueryFullProcessImageNameA(
                handle,
                PROCESS_NAME_WIN32,
                PSTR::from_raw(str.as_mut_ptr()),
                &raw mut len,
            );
            if res.is_err() {
                return None;
            }
            let name = CStr::from_ptr(str.as_ptr().cast());
            let path = OsStr::from_encoded_bytes_unchecked(name.to_bytes()).to_owned();
            Some(WindowInfo { pid, path })
        }
    }

    fn get_idle_time() -> u32 {
        unsafe {
            let now = GetTickCount();
            let last = GetLastInputInfo().unwrap().dwTime;
            now - last
        }
    }
}

impl PlatformAPI for Platform {
    fn get_activity_stream(rate: Duration) -> crate::Result<impl Stream<Item = ActivityInfo>> {
        let start = Instant::now() + rate;
        let mut interval = interval_at(start, rate);
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        let stream = unfold(interval, |mut interval| async {
            interval.tick().await;
            let activity = ActivityInfo {
                idle_time: Self::get_idle_time(),
                window: Self::get_active_window_info(),
            };
            Some((activity, interval))
        });
        Ok(stream)
    }

    fn get_all_window_info() -> crate::Result<Vec<WindowInfo>> {
        let mut map = HashMap::new();
        let _ = EnumWindows(|mut hwnd| {
            unsafe {
                let window = HWND(*hwnd.as_mut());
                if let Some(info) = Self::get_window_info(window) {
                    map.insert(info.path.clone(), info);
                }
            }
            true
        });
        Ok(map.into_values().collect())
    }
}
