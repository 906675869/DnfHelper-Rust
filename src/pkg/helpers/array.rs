// 定义函数
pub fn in_array<T :PartialEq>(check_data: T, data: &[T]) -> bool {
    for v in data {
        if check_data == *v {
            return true;
        }
    }
    false
}