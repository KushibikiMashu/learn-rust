use std::{
    io::prelude::*,
    fs::File,
    net::TcpStream,
    net::TcpListener,
};

fn main() {
    // bind関数は、新しいTcpListenerインスタンスを返すという点でnew関数のような働きをします
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incomingメソッドは、一連のストリームを与えるイテレータを返します
    // (具体的には、型TcpStreamのストリーム)
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // TCP接続へのハンドルを得ることに成功した
        println!("Connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // TCP
    // Connection established!
    // Request:

    // Chrome
    // Connection established!
    // Request: GET / HTTP/1.1
    // Host: 127.0.0.1:7878
    // Connection: keep-alive
    // Cache-Control: max-age=0
    // Upgrade-Insecure-Requests: 1
    // User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.121 Safari/537.36
    // Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8
    // Accept-Encoding: gzip, deflate, br
    // Accept-Language: ja,en-US;q=0.9,en;q=0.8

    let get = b"GET / HTTP/1.1\r\n";


    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}