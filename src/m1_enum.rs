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

#[derive(Debug)]
enum GenericOption<T>{
    None, 
    Some(T),
}

fn num_under_five(num_check: u8) -> GenericResult<u8, String> {
    if num_check < 5{
        GenericResult::Ok(num_check)
    } else {
        GenericResult::Err("Not valid".to_string())
    }
}

fn num_under_five_option(num_check: u8) -> GenericOption<u8> {
    if num_check < 5 {
        GenericOption::Some(num_check)
    } else {
        return GenericOption::None;
    }
}


/// Built in functions for Result and options 

fn num_under_five_built_in(num_check: u8) -> Result<u8, String> {
    if num_check < 5{
        Ok(num_check)
    } else {
        Err("Not valid".to_string())
    }
}
fn num_under_five_option_built_in(num_check: u8) -> Option<u8> {
    if num_check < 5 {
        Some(num_check)
    } else {
         None
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
        let under_five_res_option: GenericOption<u8> = num_under_five_option(10);
        dbg!(under_five_res_option);

        // Built in functions debug

        let under_five_res_built_in = num_under_five_built_in(2);
        dbg!(under_five_res_built_in);
        let under_five_res_option_built_in = num_under_five_option_built_in(10);
        dbg!(under_five_res_option_built_in);
    }
}
