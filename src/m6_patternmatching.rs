
#[derive(Debug)]
enum Message  {
    Quit, 
    ChangeColor(i32, i32),
    Move { x: String, y:String},
    WriteData(String),
}

fn process_message(msg: Message){
    match  msg {
        Message::Quit => println!("I quit"),
        Message::ChangeColor(x,y )=> println!("x, {} : y, {}", x, y),
        Message::Move { x, y } => println!("change name from {} to {}",x, y),
        Message::WriteData(text)=>{
            println!("text {}", text)
        },

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_match(){
        let my_quit = Message::Quit;
        process_message(my_quit);
        let my_color = Message::ChangeColor(40, 30);
        process_message(my_color);
        let my_move = Message::Move { x: "test".to_string(), y: "new".to_string() };
        process_message(my_move);
        let my_data = Message::WriteData("data".to_string());
        process_message(my_data);
    }
}

