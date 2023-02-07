use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

pub fn start_server() {
    let server = TcpListener::bind("0.0.0.0:9000").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                let msg = websocket.read_message().unwrap();

                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}
