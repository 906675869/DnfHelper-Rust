use std::{mem, ptr};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use winapi::shared::minwindef::DWORD;
use winapi::um::handleapi::CloseHandle;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
    TH32CS_SNAPPROCESS,
};

pub fn get_process_id(process_name: &str) -> DWORD {
    let mut result: DWORD = 0;
    unsafe {
        let snapshot_handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot_handle == ptr::null_mut() {
            CloseHandle(snapshot_handle);
            return result;
        }

        let mut pe: PROCESSENTRY32W = mem::zeroed();
        pe.dwSize = mem::size_of::<PROCESSENTRY32W>() as u32;

        if Process32FirstW(snapshot_handle, &mut pe) != 0 {
            loop {
                if Process32NextW(snapshot_handle, &mut pe) == 0 {
                    break;
                }
                let current_process: String = OsString::from_wide(&pe.szExeFile).to_string_lossy().to_lowercase();
                if current_process == process_name.to_lowercase() {
                    result = pe.th32ProcessID;
                    return result;
                }
            }
        }

        CloseHandle(snapshot_handle);
    }

    result
}