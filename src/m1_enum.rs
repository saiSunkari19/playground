#[derive(Debug)]
enum CarColur {
    Red,
    Blue,
    Green,
}

fn create_car_colour() -> CarColur {
    let color = CarColur::Blue;
    color
}
#[derive(Debug)]
enum GenericResult<T, E> {
    Ok(T),
    Err(E),
}
fn num_under_five(num_check: u8) -> GenericResult<u8, String> {
    if (num_check < 5) {
        GenericResult::Ok(num_check)
    } else {
        GenericResult::Err("Not valid".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_color: CarColur = create_car_colour();
        dbg!(car_color);

        let under_five_res = num_under_five(10);
        dbg!(under_five_res);
    }
}
