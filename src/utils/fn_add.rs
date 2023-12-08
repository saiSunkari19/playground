/// # Function : add
///
/// # Arguments : u32
/// # Returns : u32
/// # Examples
/// ```
/// let x = 10
/// let y = add(x);
/// ```
///
/**
 * This is the description about the add function
 * function add
 */

pub fn add(num: u32) -> u32 {
    num + 5
}

#[cfg(test)]
mod test {
    use super::add;

    #[test]
    fn add_test() {
        let x: i32 = 100;
        let y = add(x.try_into().unwrap());
        println!("x and y are {} {}", x, y);

        assert_eq!(y, 105);
    }
}
