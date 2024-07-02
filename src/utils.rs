//! 快捷函数

use std::ffi::CString;

use raylib::{
    ffi::{
        self, DrawTextEx, GetCharPressed, GetKeyPressed, GetMouseWheelMove, MeasureTextEx,
        MouseCursor,
    },
    RaylibHandle, RaylibThread,
};

/// 快速创建窗口
///
/// * `width`: 窗口宽度
/// * `height`: 窗口高度
pub fn sim_creae_windows(width: i32, height: i32) -> (RaylibHandle, RaylibThread) {
    let (mut rl, thread) = raylib::init().size(width, height).title("DTerm").build();
    // 允许窗口修改大小
    let mut state = rl.get_window_state();
    state = state.set_window_resizable(true);
    rl.set_window_state(state);
    rl.show_cursor();
    rl.set_exit_key(None);
    rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_NOT_ALLOWED);
    (rl, thread)
}

pub fn get_key_pressed() -> i32 {
    unsafe { GetKeyPressed() }
}

pub fn get_mouse_wheel_move() -> f32 {
    unsafe { GetMouseWheelMove() }
}

pub fn get_char_pressed() -> u32 {
    let s = unsafe { GetCharPressed() } as u32;
    s
}

pub fn draw_text_ex(
    font: impl AsRef<ffi::Font>,
    text: &Vec<u8>,
    position: impl Into<ffi::Vector2>,
    font_size: f32,
    spacing: f32,
    tint: impl Into<ffi::Color>,
) {
    let c_text = CString::new(text.to_owned()).unwrap();
    unsafe {
        DrawTextEx(
            *font.as_ref(),
            c_text.as_ptr(),
            position.into(),
            font_size,
            spacing,
            tint.into(),
        )
    }
}

/// 检查是否需要换行
///
/// * `chars`: 字符ascli编码串
pub fn inspect_wrap(chars: Vec<u8>) -> bool {
    if !chars.is_empty() {
        if chars.len() > 1 && chars[chars.len() - 2..].to_owned() == b"\r\n" {
            return true;
        } else {
            if let Some(&c) = chars.last() {
                if c == b'\r' || c == b'\n' {
                    return true;
                }
            }
        }
    }
    false
}

pub fn measuretextex(font: ffi::Font, text: &Vec<u8>, font_size: f32, space: f32) -> ffi::Vector2 {
    let c_text = CString::new(text.to_owned()).unwrap();
    unsafe { MeasureTextEx(font, c_text.as_ptr(), font_size, space) }
}
