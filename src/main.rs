pub mod game;
pub mod pkg;

use pkg::helpers::array;
use game::address;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let check_data = 3;
    if array::in_array(check_data, &data) {
        println!("{} is in the array.", check_data);
    } else {
        println!("{} is not in the array.", check_data);
    }

    println!("{} is",address::BbJzAddr)
}