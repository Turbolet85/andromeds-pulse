//! Xlib's thread support, initialised before any thread exists.
//!
//! The app uses Xlib from two threads: GTK's main thread, and a device-event
//! thread `tao` starts on X11. Xlib older than 1.8 does not initialise its
//! thread support when it loads, so such a process has to call
//! `XInitThreads()` before any other Xlib call. Without it a reply is lost
//! between the two threads, Xlib raises an I/O error on a healthy connection
//! and GDK's handler calls `_exit(1)` in the app's first second.
//!
//! The call is inert where the library or the symbol is not found: a host
//! with no Xlib has no X11 window path to protect.
//!
//! This module leaves with the window (the route entry `Window retired`).

use std::ffi::CStr;

pub const XLIB_LIBRARY: &CStr = c"libX11.so.6";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XlibThreads {
    Initialised,
    LibraryNotFound,
    SymbolNotFound,
    /// `XInitThreads()` returned 0.
    Refused,
    /// Not Linux.
    NotApplicable,
}

/// Initialises Xlib's thread support.
///
/// Must be called ONLY as the second statement of `main()`, directly after
/// the render-posture step: before the tokio runtime builds any thread and
/// before anything reaches Xlib.
pub fn init() -> XlibThreads {
    init_from(XLIB_LIBRARY)
}

#[doc(hidden)]
#[cfg(target_os = "linux")]
pub fn init_from(library: &CStr) -> XlibThreads {
    // SAFETY: `library` is NUL-terminated and `dlopen` has no other
    // precondition. The handle is never closed, so what it maps stays mapped.
    let handle = unsafe { libc::dlopen(library.as_ptr(), libc::RTLD_NOW) };
    if handle.is_null() {
        return XlibThreads::LibraryNotFound;
    }
    // SAFETY: `handle` is the live handle `dlopen` just returned and the
    // symbol name is NUL-terminated.
    let symbol = unsafe { libc::dlsym(handle, c"XInitThreads".as_ptr()) };
    if symbol.is_null() {
        return XlibThreads::SymbolNotFound;
    }
    // SAFETY: Xlib declares `Status XInitThreads(void)` with `Status` an
    // `int`, and `symbol` is that function's non-null address in the library
    // opened above.
    let init_threads: unsafe extern "C" fn() -> libc::c_int =
        unsafe { std::mem::transmute(symbol) };
    // SAFETY: the function takes no argument and may be called more than
    // once; `init` is called while the process has one thread.
    match unsafe { init_threads() } {
        0 => XlibThreads::Refused,
        _ => XlibThreads::Initialised,
    }
}

#[doc(hidden)]
#[cfg(not(target_os = "linux"))]
pub fn init_from(_library: &CStr) -> XlibThreads {
    XlibThreads::NotApplicable
}
