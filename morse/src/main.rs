mod code_parser;
mod user_input;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
//use hyper::server::{Server, Response};

fn define_address() -> SocketAddr {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    return address
}

fn main() {
    println!("Welcome to Maxwell Flitton's morse code cipher");
    let out_come = user_input::console_input::get_input();
    let test_struct = code_parser::writer::Word::new(&out_come.to_string());
    let message = test_struct.compile();
//    test_struct.display();
    for i in &message {
        print!("=>{}", i);
    }
    println!(":");
    let recieved_message = code_parser::reader::ReceivedMessage::new(&message);
    recieved_message.display();

    let _address = define_address();
//    let builder = Server::bind(&address);
//    let server = builder.serve(|| {
//        service_fn_ok(|_| {
//            Response::new(Body::from("Almost microservice..."))
//        })
//    });
}

#[cfg(test)]
mod main_tests {

    use super::define_address;
    use std::net:: {IpAddr, Ipv4Addr};

    #[test]
    fn test_address() {
        let socket = define_address();
        assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(socket.port(), 8080);
    }

}
