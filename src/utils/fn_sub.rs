pub fn sub(num: u32) -> u32 {
    num - 5
}

#[cfg(test)]
mod test {
    use super::sub;

    #[test]
    fn sub_test() {
        let x = 100;
        let y = sub(x);
        println!("x and y are {} {} ", x, y);
        assert_eq!(y, 95);
    }
}
