// 定义 trait
pub trait Driver {
    fn load_driver(&mut self, driver_file_path: &str, service_name: &str, display_name: &str) -> bool;
    fn unload_driver(&self) -> bool;
    fn alloc_memory(&self, len: u32) -> u64;
    fn set_global_id(&self, pid: u32);
    fn read_int(&self, address: u64) -> u32;
    fn read_int64(&self, address: u64) -> u64;
    fn read_float32(&self, address: u64) -> f32;
    fn read_float64(&self, address: u64) -> f64;
    fn read_byte_arr(&self, address: u64, length: u32) -> Vec<u8>;
    fn write_int(&self, address: u64, value: u32) -> bool;
    fn write_int64(&self, address: u64, value: u64) -> bool;
    fn write_float32(&self, address: u64, value: f32) -> bool;
    fn write_float64(&self, address: u64, value: f64) -> bool;
    fn write_byte_arr(&self, address: u64, value: &[u8]) -> bool;
}
