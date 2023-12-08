fn add(num: u32) -> u32 {
    num + 5
}

fn sum_of_array(arr: &[i32]) -> i32{
    arr.iter().sum()
}

fn main(){
    let x: i32 = 20;

    let y = add(x.try_into().unwrap());

    println!("x is {}", x);
    println!("y is {}", y);

    let arr = [1,2,3,4];

    println!("Sum of array {}", sum_of_array(&arr));
}
