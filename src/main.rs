use windows::{
    core::*, Win32::Foundation::*,
    Win32::{UI::{WindowsAndMessaging::*, Input::KeyboardAndMouse::*}, System::SystemInformation::GetTickCount},
};
use std::{thread, time};
use chrono::Utc;

fn main() -> Result<()> {
    let mut start_time = Utc::now();

    unsafe {        
        let mut window_text: [u16; 1024] = [0; 1024];
        // let mut proc_text: [u16; 1024] = [0; 1024];
        let mut prev_window = String::new();
        loop {
            let hwnd = GetForegroundWindow();
            // Failed to use the WindowTextLength variable:
            // let lgth = GetWindowTextLengthW(hwnd);            
            // let mut text = Vec::<u16>::with_capacity(lgth.try_into().unwrap());       
            // let len = GetWindowTextW(hwnd, PWSTR(text.as_mut_ptr()), text.capacity() as i32);
            let len = GetWindowTextW(hwnd, PWSTR(window_text.as_mut_ptr()), window_text.len() as i32);
            let window = String::from_utf16_lossy(&window_text[..len as usize]);
            
            // Failed to get the file being executed by the foreground window's process:
            // let mut pid_buf: u32 = 0;
            // let pid_buf: *mut u32 = &mut pid_buf;
            // let pid = GetWindowThreadProcessId(hwnd, pid_buf);
            // let proc = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
            // println!("  {}", proc.is_invalid());
            // let len = K32GetModuleBaseNameW(proc, None, PWSTR(proc_text.as_mut_ptr()), proc_text.len() as u32);            
            // let text = String::from_utf16_lossy(&proc_text[..len as usize]);
            // let len = K32GetModuleFileNameExW(proc, None, PWSTR(proc_text.as_mut_ptr()), proc_text.len() as u32);
            // let len = GetWindowModuleFileNameW(hwnd, PWSTR(proc_text.as_mut_ptr()), proc_text.len() as u32);
            // let text = String::from_utf16_lossy(&proc_text[..len as usize]);
            
            let mut last_input_info = LASTINPUTINFO {
                cbSize: 8, 
                dwTime: 0
            };        
            let p_last_input_info = &mut last_input_info as *mut LASTINPUTINFO;
            let res = GetLastInputInfo(p_last_input_info);
            if res.as_bool() {
                let idle_min = ((GetTickCount() - last_input_info.dwTime)/1000)/60;                
                if idle_min >= 5 && idle_min % 5 == 0 {
                    println!("{}: Idle (i.e. no mouse/keyboard activities) for: {} minutes\n", Utc::now().to_rfc2822(), idle_min);
                }
            }
            if !window.is_empty() {                
                if prev_window != window {
                    let end_time = Utc::now();
                    let diff = end_time.time() - start_time.time();
                    println!("{} minutes ({} seconds) was spent in [{}]", diff.num_minutes(), diff.num_seconds(), prev_window);
                    println!("{}: using [{}]\n", Utc::now().to_rfc2822(), window);
                    prev_window = window;
                    start_time = end_time;
                }
            }
            thread::sleep(time::Duration::from_secs(60));
        }
    }
}