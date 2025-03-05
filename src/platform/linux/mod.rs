mod error;

use futures::stream::unfold;
use futures_core::Stream;
use std::{ffi::OsString, fs::read_link, time::Duration};
use tokio::time::{interval_at, Instant, MissedTickBehavior};
use x11rb::{
    connection::Connection,
    protocol::{
        screensaver::query_info,
        xproto::{intern_atom, AtomEnum, ConnectionExt},
    },
    rust_connection::RustConnection,
};

use super::{ActivityInfo, PlatformAPI, WindowInfo};

pub use error::*;

pub struct PlatformConnection {
    conn: RustConnection,
    root: u32,
    atom_client_list: u32,
    atom_pid: u32,
}

impl PlatformConnection {
    pub fn new() -> Result<Self> {
        let (conn, _) = x11rb::connect(None)?;
        let atom_client_list = intern_atom(&conn, true, b"_NET_CLIENT_LIST")?.reply()?.atom;
        let atom_pid = intern_atom(&conn, true, b"_NET_WM_PID")?.reply()?.atom;
        let root = conn.setup().roots[0].root;
        Ok(Self {
            conn,
            root,
            atom_client_list,
            atom_pid,
        })
    }

    fn get_activity_info(&self) -> Result<ActivityInfo> {
        let window = self.get_active_window_info();
        let idle_time = self.get_idle_time()?;

        Ok(ActivityInfo { window, idle_time })
    }

    fn get_active_window_info(&self) -> Option<WindowInfo> {
        let window = self.conn.get_input_focus().ok()?.reply().ok()?.focus;

        self.get_window_info(window)
    }

    fn get_all_window_info(&self) -> Result<Vec<WindowInfo>> {
        let reply = self
            .conn
            .get_property(
                false,
                self.root,
                self.atom_client_list,
                AtomEnum::ANY,
                0,
                u32::MAX,
            )?
            .reply()?;
        let Some(values) = reply.value32() else {
            return Ok(Vec::new());
        };
        Ok(values
            .filter_map(|window| self.get_window_info(window))
            .collect())
    }

    fn get_window_info(&self, window: u32) -> Option<WindowInfo> {
        let pid = self.get_window_pid(window)?;
        let path = get_process_path(pid).ok()?;

        Some(WindowInfo { pid, path })
    }

    fn get_idle_time(&self) -> Result<u32> {
        let query = query_info(&self.conn, self.root)?.reply()?;
        Ok(query.ms_since_user_input)
    }

    fn get_window_pid(&self, window: u32) -> Option<u32> {
        self.conn
            .get_property(false, window, self.atom_pid, AtomEnum::ANY, 0, u32::MAX)
            .ok()?
            .reply()
            .ok()?
            .value32()?
            .next()
    }
}

pub struct Platform;

impl PlatformAPI for Platform {
    fn get_activity_stream(rate: Duration) -> crate::Result<impl Stream<Item = ActivityInfo>> {
        let start = Instant::now() + rate;
        let mut interval = interval_at(start, rate);
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        let connection = PlatformConnection::new()?;
        let stream = unfold((connection, interval), |(connection, mut interval)| async {
            interval.tick().await;
            connection
                .get_activity_info()
                .map(|activity| (activity, (connection, interval)))
                .ok()
        });
        Ok(stream)
    }

    fn get_all_window_info() -> crate::Result<Vec<WindowInfo>> {
        let connection = PlatformConnection::new()?;
        Ok(connection.get_all_window_info()?)
    }
}

fn get_process_path(pid: u32) -> crate::Result<OsString> {
    read_link(format!("/proc/{pid}/exe"))
        .map(Into::into)
        .map_err(|_| crate::Error::ProcessPathNotFound(pid))
}
