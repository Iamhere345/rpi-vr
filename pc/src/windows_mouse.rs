use std::mem::size_of;
use windows::{
    self,
    Win32::UI::{
        Input::KeyboardAndMouse,
        WindowsAndMessaging::{self, GetSystemMetrics},
    },
};

#[allow(non_snake_case)]
enum WindowsSendInputEnum {
    Keyboard {
        wVk: KeyboardAndMouse::VIRTUAL_KEY,
        wScan: u16,
        dwFlags: KeyboardAndMouse::KEYBD_EVENT_FLAGS,
    },
    Mouse {
        dx: i32,
        dy: i32,
        mouseData: i32,
        dwFlags: KeyboardAndMouse::MOUSE_EVENT_FLAGS,
    },
    #[allow(unused)]
    Hardware {
        uMsg: u32,
        wParamL: u16,
        wParamH: u16,
    },
}

impl WindowsSendInputEnum {
    pub fn into_windows(self) -> KeyboardAndMouse::INPUT {
        let (a, b) = match self {
            WindowsSendInputEnum::Keyboard {
                wVk,
                wScan,
                dwFlags,
            } => (
                KeyboardAndMouse::INPUT_KEYBOARD,
                KeyboardAndMouse::INPUT_0 {
                    ki: KeyboardAndMouse::KEYBDINPUT {
                        wVk,
                        wScan,
                        dwFlags,
                        time: 0,
                        dwExtraInfo: get_message_extra_info(),
                    },
                },
            ),
            WindowsSendInputEnum::Mouse {
                dx,
                dy,
                mouseData,
                dwFlags,
            } => (
                KeyboardAndMouse::INPUT_MOUSE,
                KeyboardAndMouse::INPUT_0 {
                    mi: KeyboardAndMouse::MOUSEINPUT {
                        dx,
                        dy,
                        mouseData,
                        dwFlags,
                        time: 0,
                        dwExtraInfo: get_message_extra_info(),
                    },
                },
            ),
            WindowsSendInputEnum::Hardware {
                uMsg,
                wParamL,
                wParamH,
            } => (
                KeyboardAndMouse::INPUT_HARDWARE,
                KeyboardAndMouse::INPUT_0 {
                    hi: KeyboardAndMouse::HARDWAREINPUT {
                        uMsg,
                        wParamL,
                        wParamH,
                    },
                },
            ),
        };
        KeyboardAndMouse::INPUT {
            r#type: a,
            Anonymous: b,
        }
    }
}

fn send_input(inputs: &[KeyboardAndMouse::INPUT]) {
    unsafe { KeyboardAndMouse::SendInput(inputs, size_of::<KeyboardAndMouse::INPUT>() as i32) };
}

fn get_message_extra_info() -> usize {
    unsafe { WindowsAndMessaging::GetMessageExtraInfo() }.0 as usize
}

#[allow(unused)]
fn primary_screen_size() -> (i32, i32) {
    // SAFETY: calls has no dangerous side-effects
    let x = unsafe { GetSystemMetrics(WindowsAndMessaging::SM_CXSCREEN) };
    let y = unsafe { GetSystemMetrics(WindowsAndMessaging::SM_CYSCREEN) };
    (x, y)
}

fn virtual_screen_size() -> (i32, i32) {
    // SAFETY: calls has no dangerous side-effects
    let x = unsafe { GetSystemMetrics(WindowsAndMessaging::SM_CXVIRTUALSCREEN) };
    let y = unsafe { GetSystemMetrics(WindowsAndMessaging::SM_CYVIRTUALSCREEN) };
    (x, y)
}
/// `x` and `y` contain normalized absolute coordinates between 0 and 65,535.
/// The event procedure maps these coordinates onto the display surface.
/// Coordinate (0,0) maps onto the upper-left corner of the display surface;
/// coordinate (65535,65535) maps onto the lower-right corner.
/// In a multimonitor system, the coordinates map to the primary monitor.
fn mouse_move_to(x: i32, y: i32) {
    send_input(&[WindowsSendInputEnum::Mouse {
        dx: x,
        dy: y,
        mouseData: 0,
        dwFlags: KeyboardAndMouse::MOUSEEVENTF_MOVE | KeyboardAndMouse::MOUSEEVENTF_ABSOLUTE,
    }
    .into_windows()]);
}

/// `x` and `y` now corresponds to screen width and height
pub fn denormalised_mouse_move_to(x: i32, y: i32) {
    let (screen_size_x, screen_size_y) = primary_screen_size();
    let x = x * 65535 / screen_size_x;
    let y = y * 65535 / screen_size_y;
    mouse_move_to(x, y);
}

/// same as [`mouse_move_to`] but the coordinates map to the entire virtual desktop.
pub fn virtual_desktop_mouse_move_to(x: i32, y: i32) {
    send_input(&[WindowsSendInputEnum::Mouse {
        dx: x,
        dy: y,
        mouseData: 0,
        dwFlags: KeyboardAndMouse::MOUSEEVENTF_MOVE
            | KeyboardAndMouse::MOUSEEVENTF_ABSOLUTE
            | KeyboardAndMouse::MOUSEEVENTF_VIRTUALDESK,
    }
    .into_windows()]);
}

/// same as [`mouse_move_to`] but the coordinates map to the entire virtual desktop.
pub fn virtual_desktop_denormalised_mouse_move_to(x: i32, y: i32) {
    let (screen_size_x, screen_size_y) = virtual_screen_size();
    let x = x * 65535 / screen_size_x;
    let y = y * 65535 / screen_size_y;
    virtual_desktop_mouse_move_to(x, y);
}