use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
// use std::thread;

use rust_httpserver::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let mut buf: [u8; 512] = [0; 512];
    _ = stream.read(&mut buf).unwrap();
    println!("  => Request:\r\n{}", String::from_utf8_lossy(&buf[..]));

    // http protocal, ref
    //   https://www.xiexianbin.cn/http/index.html
    //   https://www.xiexianbin.cn/golang/web/index.html
    let response_content_type: &str = "content-type: text/html; charset=utf-8";
    let index = b"GET / HTTP/1.1\r\n";
    let (resp_code, filename) = if buf.starts_with(index) {
        ("HTTP/1.1 200 OK", "./static/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./static/404.html")
    };
    let content: String = fs::read_to_string(filename).unwrap();
    let response: String = format!(
        "{}\r\n{}\r\n\r\n{}",
        resp_code, response_content_type, content
    );
    println!("  => Response:\r\n{}", response);

    _ = stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let addr: &str = "0.0.0.0:8000";
    let listener: TcpListener = TcpListener::bind(addr).unwrap();
    println!("License on {}", addr);

    // thread pool
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        // handle_connection(stream);
        // println!("Connection Established!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.")
}
