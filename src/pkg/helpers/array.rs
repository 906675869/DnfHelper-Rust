// 检测数组中是否存在数值
// let data = vec![1, 2, 3, 4, 5];
// let check_data = 3;
pub fn in_array<T :PartialEq>(check_data: T, data: &[T]) -> bool {
    for v in data {
        if check_data == *v {
            return true;
        }
    }
    false
}