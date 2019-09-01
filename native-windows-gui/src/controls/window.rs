use winapi::um::winuser::{WS_OVERLAPPEDWINDOW, WS_CLIPCHILDREN, WS_VISIBLE, WS_DISABLED, WS_MAXIMIZE, WS_MINIMIZE, WS_CAPTION,
WS_MINIMIZEBOX, WS_MAXIMIZEBOX, WS_SYSMENU, WS_THICKFRAME};

use crate::win32::window_helper as wh;
use super::ControlHandle;

const NOT_BOUND: &'static str = "Window is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Window handle is not HWND!";


bitflags! {
    pub struct WindowFlags: u32 {
        const MAIN_WINDOW = WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_THICKFRAME | WS_MAXIMIZEBOX;
        const WINDOW = WS_CAPTION | WS_SYSMENU;
        const MINIMIZE_BOX = WS_MINIMIZEBOX;
        const MAXIMIZE_BOX = WS_MAXIMIZEBOX;
        const SYS_MENU = WS_SYSMENU;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const MAXIMIZED = WS_MAXIMIZE;
        const MINIMIZED = WS_MINIMIZE;
        const RESIZABLE = WS_THICKFRAME | WS_MAXIMIZEBOX;
    }
}

#[derive(Default, Debug)]
pub struct Window {
    pub handle: ControlHandle
}

impl Window {

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return window title
    pub fn text(&self) -> String { 
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the window title
    pub fn set_text<'a>(&self, v: &'a str) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        None
    }

    // Winapi base flags used during window creation
    pub fn flags(&self) -> (u32, u32) {
        (WS_OVERLAPPEDWINDOW | WS_VISIBLE, 0)
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        WS_CLIPCHILDREN
    }
}
