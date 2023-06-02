use winapi::{
    shared::ntdef::{HANDLE, NULL},
    um::{
        memoryapi::{ReadProcessMemory, WriteProcessMemory},
        processthreadsapi::OpenProcess,
        winnt::{PROCESS_ALL_ACCESS, PVOID},
    },
};

pub struct MemoryRw {
    pid: u32,            // 进程id
    handle: HANDLE,      // 进程句柄
}

impl MemoryRw {
    pub fn new() -> Self {
        Self {
            pid: 0,
            handle: NULL,
        }
    }

    pub fn set_global_id(&mut self, pid: u32) {
        self.pid = pid;
        let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false as i32, pid) };
        if handle.is_null() {
            return;
        }
        self.handle = handle;
    }

    pub fn read_process_memory(&self, address: PVOID, buffer: &mut [u8]) -> bool {
        let mut bytes_read = 0;
        let result = unsafe {
            ReadProcessMemory(
                self.handle,
                address,
                buffer.as_mut_ptr() as PVOID,
                buffer.len(),
                &mut bytes_read,
            )
        };
        result != 0 && bytes_read == buffer.len()
    }

    pub fn write_process_memory(&self, address: PVOID, buffer: &[u8]) -> bool {
        let mut bytes_written = 0;
        let result = unsafe {
            WriteProcessMemory(
                self.handle,
                address,
                buffer.as_ptr() as PVOID,
                buffer.len(),
                &mut bytes_written,
            )
        };
        result != 0 && bytes_written == buffer.len()
    }
}