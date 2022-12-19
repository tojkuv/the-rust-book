use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.0:7878").unwrap(); // returns a `TcpListener` instance
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        // `incoming` returns an iterator of 'TcpStream` instances that represent external client connection attempts
        let stream = stream.unwrap(); // this is how we handle a failed connection attempt

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    //    let http_request: Vec<_> = buf_reader
    //            .lines() /* this is a function from the `BufRead` trait. it return an iterator strings, separated by the line breaks */
    //            .map(|result| result.unwrap()) /* unwrap each result */
    //            .take_while(|line| !line.is_empty()) /* browers end requests with two `new-lines`. */
    //            .collect(); /* collect all elements into a vector */
    // unwrapping a result after unwrapping an option - if the option type is None,then the first unwrap panics
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    //    if request_line == "GET / HTTP/1.1" {
    //        let status_line = "HTTP/1.1 200 OK";
    //        let contents = fs::read_to_string("hello.html").unwrap();
    //        let length = contents.len(); // this length is used for the `contents-length` header
    //
    //        let response = format!("{status_line}\r\nContents-Length: {length}\r\n\r\n{contents}"); // build the string that will be used for the http response
    //
    //        stream.write_all(response.as_bytes()).unwrap(); // convert the string to bytes, pass the bytes as an argument to `write_all` and unwrap the result
    //    } else {
    //        let status_line = "HHTP/1.1 404 NOT FOUND";
    //        let contents = fs::read_to_string("404.html").unwrap();
    //
    //        let response = format!("{status_line}\r\nContent-Lenght: {length}\r\n\r\n{contents}");
    //
    //        stream.write_all(response.as_bytes()).unwrap();
    //    }

    // match does not do automatic dereferencing, because it is not always desired
    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    //design the public api, then implement the functionality
}
