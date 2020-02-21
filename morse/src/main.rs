#![feature(proc_macro_hygiene, decl_macro)]
mod code_parser;
mod user_input;
mod file_manager;

#[macro_use] extern crate rocket;

# [get("/")]
fn home() -> String {
    return "it's working".to_string()
}


fn main() {
    println!("Welcome to Maxwell Flitton's morse code cipher");
    let out_come = user_input::console_input::get_input();
    let mut test_struct = code_parser::writer::Word::new(&out_come.to_string());
    test_struct.contained_compile();

    for i in &test_struct.compiled_message {
        print!("=>{}", i);
    }

    let mut written_message = String::new();

    for i in &test_struct.compiled_message {
        written_message.push("=".chars().next().unwrap());
        written_message.push(">".chars().next().unwrap());

        for x in i.to_string().chars() {
            written_message.push(x);
        }
    }

    let _ = file_manager::file_handler::write(written_message);

    let recieved_message = code_parser::reader::ReceivedMessage::new(&test_struct.compiled_message);
    recieved_message.display();
    rocket::ignite().mount("/", routes![home]).launch();
}
