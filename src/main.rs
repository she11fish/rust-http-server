use std::net::{TcpListener, TcpStream};
use std::str;
mod controller;
use controller::Controller;
const PORT: &str = "8000";


fn handle_client(stream: TcpStream) {
    let mut controller = Controller { stream , port: PORT };
    controller.route_handler();

}


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{PORT}")).unwrap();
    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(socket) => handle_client(socket),
            Err(_) => { println!("HOWER"); panic!("ASFA")},
        };
    }
    Ok(())
}