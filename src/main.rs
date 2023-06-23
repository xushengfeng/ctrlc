extern crate winapi;
use winapi::um::winuser::{keybd_event, VK_CONTROL};
use std::thread;
use std::time::Duration;

fn main() {
    unsafe {
        // 模拟按下 Ctrl 键
        keybd_event(VK_CONTROL as u8, 0, 0, 0);
        // 模拟按下 C 键（C 键的虚拟键码为 67）
        keybd_event(67, 0, 0, 0);

        // 等待一段时间，模拟按键保持按下的效果
        thread::sleep(Duration::from_millis(100));

        // 模拟释放 C 键
        keybd_event(67, 0, winapi::um::winuser::KEYEVENTF_KEYUP, 0);
        // 模拟释放 Ctrl 键
        keybd_event(VK_CONTROL as u8, 0, winapi::um::winuser::KEYEVENTF_KEYUP, 0);
    }
}
