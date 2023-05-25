use winapi::shared::minwindef::{LPVOID, DWORD};
use winapi::um::processthreadsapi::{OpenProcess};
use winapi::um::handleapi::{CloseHandle};
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory, VirtualAllocEx};
use winapi::um::winnt::{PROCESS_ALL_ACCESS,MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};

pub fn virtual_alloc_ex(process_id: DWORD) -> LPVOID {
    // 打开进程句柄
    let process_handle = unsafe {
        OpenProcess(
            PROCESS_ALL_ACCESS,  // 所有权限
            false as i32,  // 不继承句柄
            process_id,  // 进程 ID
        )
    };
    if process_handle.is_null() {
        return 0 as LPVOID;
    }

    let address = unsafe {
        VirtualAllocEx(
            process_handle,
            std::ptr::null_mut(),
            4096,
            MEM_RESERVE | MEM_COMMIT,
            PAGE_READWRITE,
        )
    };
    address
}

pub fn read_process_memory(process_id: DWORD, address: LPVOID, buffer: &mut [u8]) -> bool {
    // 打开进程句柄
    let process_handle = unsafe {
        OpenProcess(
            PROCESS_ALL_ACCESS,  // 所有权限
            false as i32,  // 不继承句柄
            process_id,  // 进程 ID
        )
    };
    if process_handle.is_null() {
        return false;
    }

    // 读取进程内存
    let mut bytes_read: usize = 0;
    let success = unsafe {
        ReadProcessMemory(
            process_handle,  // 进程句柄
            address,  // 内存地址
            buffer.as_mut_ptr() as LPVOID,  // 缓冲区地址
            buffer.len(),  // 缓冲区大小
            &mut bytes_read,  // 实际读取的字节数
        )
    };
    if success == 0 {
        return false;
    }

    // 关闭进程句柄
    unsafe {
        CloseHandle(process_handle);
    }

    true
}

pub fn write_process_memory(process_id: DWORD, address: LPVOID, buffer: &[u8]) -> bool {
    // 打开进程句柄
    let process_handle = unsafe {
        OpenProcess(
            PROCESS_ALL_ACCESS,  // 所有权限
            false as i32,  // 不继承句柄
            process_id,  // 进程 ID
        )
    };
    if process_handle.is_null() {
        return false;
    }

    // 写入进程内存
    let mut bytes_written: usize = 0;
    let success = unsafe {
        WriteProcessMemory(
            process_handle,  // 进程句柄
            address,  // 内存地址
            buffer.as_ptr() as LPVOID,  // 缓冲区地址
            buffer.len(),  // 缓冲区大小
            &mut bytes_written,  // 实际写入的字节数
        )
    };
    if success == 0 {
        return false;
    }

    // 关闭进程句柄
    unsafe {
        CloseHandle(process_handle);
    }

    true
}
