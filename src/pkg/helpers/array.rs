// 定义函数
fn in_array<T>(check_data: T, data: &[T]) -> bool {
    for v in data {
        if check_data == *v {
            return true;
        }
    }
    false
}