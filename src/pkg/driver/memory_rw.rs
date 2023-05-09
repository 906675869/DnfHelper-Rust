pub struct MemoryRw {}

impl Driver for MemoryRw {
    fn load_driver(&mut self, driver_file_path: &str, service_name: &str, display_name: &str) -> bool {
        false
    }
}