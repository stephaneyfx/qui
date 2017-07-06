// Copyright (C) 2017 Stephane Raux. Distributed under the MIT license.

//! QML application module

use clue;
use clue::convert::{from_string_view_lossy, get_ffi_value, to_string_view};
use log::LogLevel;
use qlue;
use std::env;
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::os::raw::{c_char, c_int, c_uint};
use std::panic::catch_unwind;
use std::str;
use std::sync::{Arc, RwLock, Weak};

/// QML application type
///
/// This type is neither `Send` nor `Sync` but it can hand out references to
/// itself which are. All such `AppRef` references must be detroyed before the
/// `App` instance is.
#[derive(Debug)]
pub struct App {
    r: AppRef,
    not_send_sync: PhantomData<*const ()>,
}

impl App {
    /// Instantiates a Qt application suitable to use QML.
    ///
    /// Only one instance of `App` can exist at any given time.
    ///
    /// The type of application created is currently a `QGuiApplication`, but
    /// this may change and should not be relied on.
    ///
    /// # Panics
    /// Panicks if there is already an `App` instance.
    pub fn new() -> App {
        let mut wextra = APP_EXTRA.write().unwrap();
        assert!(wextra.upgrade().is_none(), "There can be only one instance \
            of a QML application.");
        let mut extra = Arc::new(AppExtra::new());
        {
            let extra = Arc::get_mut(&mut extra).unwrap();
            unsafe {
                qlue::qlueAppNew(&mut extra.argc, extra.argv.as_mut_ptr(),
                    Some(log));
            }
        }
        *wextra = Arc::downgrade(&extra);
        App {r: AppRef::new(extra), not_send_sync: PhantomData}
    }

    /// Gets a new reference to the application instance.
    pub fn new_ref(&self) -> AppRef {
        self.r.clone()
    }

    /// Runs the application event loop and returns its exit code.
    pub fn exec(&self) -> i32 {
        unsafe {qlue::qlueAppExec() as i32}
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let mut wextra = APP_EXTRA.write().unwrap();
        let rc = Arc::strong_count(&self.r.0);
        if rc > 1 {
            // Leak the QML application instance as it should die in the thread
            // that created it, but this cannot be done here as the app is still
            // referenced.
            mem::forget(self.r.clone());
            assert_eq!(1, rc, "Cannot delete QML application while it is \
                still referenced.");
        }
        *wextra = Weak::new();
        unsafe {qlue::qlueAppDelete();}
    }
}

impl Deref for App {
    type Target = AppRef;

    fn deref(&self) -> &AppRef {
        &self.r
    }
}

/// Shared reference to the `App` instance.
///
/// This allows to use the QML application from other threads. All `AppRef`
/// references must be destroyed before the `App` instance is.
#[derive(Debug)]
pub struct AppRef(Arc<AppExtra>);

impl AppRef {
    fn new(extra: Arc<AppExtra>) -> AppRef {
        AppRef(extra)
    }

    /// Returns an `AppRef` instance if there is an existing `App` instance.
    pub fn get() -> Option<AppRef> {
        let extra = match APP_EXTRA.try_read() {
            Ok(extra) => extra,
            Err(_) => return None,
        };
        extra.upgrade().map(AppRef::new)
    }

    /// Returns the QML application name.
    pub fn name(&self) -> String {
        unsafe {
            get_ffi_value(|env, cb| qlue::qlueAppName(env, cb)).unwrap()
        }
    }

    /// Sets the QML application name.
    pub fn set_name(&self, s: &str) {
        unsafe {qlue::qlueAppSetName(to_string_view(s));}
    }

    /// Sets the Qt QuickControls2 style.
    ///
    /// The style must be set before loading QML components and cannot be
    /// changed afterwards.
    pub fn set_style(&self, s: &str) {
        unsafe {qlue::qlueAppSetStyle(to_string_view(s));}
    }
}

impl Clone for AppRef {
    fn clone(&self) -> AppRef {
        AppRef::new(self.0.clone())
    }
}

#[derive(Debug)]
struct AppExtra {
    argc: c_int,
    args: Vec<Vec<u8>>,
    argv: Vec<*mut c_char>,
}

unsafe impl Send for AppExtra {}
unsafe impl Sync for AppExtra {}

impl AppExtra {
    fn new() -> AppExtra {
        let mut args = env::args()
            .map(|s| s.into_bytes())
            .collect::<Vec<_>>();
        let argv = args.iter_mut()
            .map(|s| s.as_mut_ptr() as *mut c_char)
            .collect();
        AppExtra {
            argc: args.len() as c_int,
            args,
            argv,
        }
    }
}

lazy_static! {
    static ref APP_EXTRA: RwLock<Weak<AppExtra>> = RwLock::new(Weak::new());
}

extern fn log(level: clue::ClueLogLevel, msg: clue::ClueStringView,
        file: clue::ClueStringView, line: c_uint,
        target: clue::ClueStringView) {
    let _ = catch_unwind(|| forward_log(level, msg, file, line, target));
}

fn forward_log(level: clue::ClueLogLevel, msg: clue::ClueStringView,
        file: clue::ClueStringView, line: c_uint,
        target: clue::ClueStringView) {
    let level = match level {
        clue::ClueLogLevel::ClueLogLevelError => LogLevel::Error,
        clue::ClueLogLevel::ClueLogLevelWarn => LogLevel::Warn,
        clue::ClueLogLevel::ClueLogLevelInfo => LogLevel::Info,
        clue::ClueLogLevel::ClueLogLevelDebug => LogLevel::Debug,
        clue::ClueLogLevel::ClueLogLevelTrace => LogLevel::Trace,
    };
    unsafe {
        log!(target: &from_string_view_lossy(target), level, "[{}:{}] {}",
            from_string_view_lossy(file), line, from_string_view_lossy(msg));
    }
}
