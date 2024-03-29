mod fn_sum;
mod m1_enum;
mod m2_structs;
mod m3_traits;
mod m4_polymorphisam;
mod m6_patternmatching;
use crate::fn_sum::sum_of_array;

mod utils;

use crate::utils::fn_add::add;

fn main() {
    let x: i32 = 20;

    let y = add(x.try_into().unwrap());

    println!("x is {}", x);
    println!("y is {}", y);

    let arr = [1, 2, 3, 4];

    println!("Sum of array {}", sum_of_array(&arr));
}
