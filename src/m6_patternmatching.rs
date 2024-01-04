#[derive(Debug)]
enum Message {
    Quit,
    ChangeColor(i32, i32),
    Move { x: String, y: String },
    WriteData(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("I quit"),
        Message::ChangeColor(x, y) => println!("x, {} : y, {}", x, y),
        Message::Move { x, y } => println!("change name from {} to {}", x, y),
        Message::WriteData(text) => {
            println!("text {}", text)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_match() {
        let my_quit = Message::Quit;
        process_message(my_quit);
        let my_color = Message::ChangeColor(40, 30);
        process_message(my_color);
        let my_move = Message::Move {
            x: "test".to_string(),
            y: "new".to_string(),
        };
        process_message(my_move);
        let my_data = Message::WriteData("data".to_string());
        process_message(my_data);
    }

    #[test]
    fn test_match_guard() {
        let pair = (-2, 2);

        match pair {
            (x, y) if x == y => print!("equal"),
            (x, _) if x == 2 => print!("only x"),
            _ => println!("not bothered"),
        }
    }

    #[test]
    fn test_match_structs() {
        struct Location {
            x: i32,
            y: i32,
        };

        let location = Location { x: 5, y: 0 };
        match location {
            Location { x, y } if x == 0 => print!("on x axis"),
            Location { x, y } if y > 0 => print!("above y axis"),
            _ => println!("nope"),
        }
    }
}
