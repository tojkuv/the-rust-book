use std::net::TcpListener;

fn main() {
    // is possible for a binding to fail, thus the return type is a `Result`. in networking, this is called binding to
    // a port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for  stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
