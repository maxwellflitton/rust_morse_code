#![feature(proc_macro_hygiene, decl_macro)]
mod code_parser;
mod user_input;
mod file_manager;

#[macro_use] extern crate rocket;

# [get("/")]
fn home() -> String {
    let compiled_message = file_manager::file_handler::read_data();
    return compiled_message.to_string()
}

# [get("/update/<message>")]
fn update(message: String) -> String {
    let mut word_struct = code_parser::writer::Word::new(&message);
    word_struct.contained_compile();

    let written_message = &word_struct.construct_message();
    let _ = file_manager::file_handler::write(written_message.to_string());
    return "Message has been updated".to_string()
}

fn main() {
    println!("Welcome to Maxwell Flitton's morse code cipher");
    let out_come = user_input::console_input::get_input();

    let mut word_struct = code_parser::writer::Word::new(&out_come.to_string());
    word_struct.contained_compile();
    &word_struct.display();

    let written_message = &word_struct.construct_message();
    let _ = file_manager::file_handler::write(written_message.to_string());

    let recieved_message = code_parser::reader::ReceivedMessage::new(&word_struct.compiled_message);
    recieved_message.display();
    rocket::ignite().mount("/", routes![home, update]).launch();
}
