mod code_parser;
mod user_input;

fn main() {
    println!("Welcome to Maxwell Flitton's morse code cipher");
    let out_come = user_input::console_input::get_input();
    let test_struct = code_parser::writer::Word::new(&out_come.to_string());
    let message = test_struct.compile();
    for i in &message {
        print!("=>{}", i);
    }
    println!(":");
    let recieved_message = code_parser::reader::ReceivedMessage::new(&message);
    recieved_message.display();
}
