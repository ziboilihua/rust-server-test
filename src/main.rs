use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    // 在本地7878端口创建TCP连接
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 监听TCP连接
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}


fn handle_connection(mut stream: TcpStream) {
    // 在栈上声明一个 buffer 来存放读取到的数据，创建缓冲区的大小为1024字节
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // 将缓冲区的字节转换为字符串
    let content = String::from_utf8_lossy(&buffer);
    // 打印请求内容
    print!("{}", content);
    write_response(stream, content.as_ref());
}

fn write_response(mut stream: TcpStream, content: &str) {
    // 获取根据请求内容返回的Result枚举
    let result: Result<HttpMethod, &str> = handle_http_method(content);
    // 模式匹配判断是否为Http请求将返回值赋值给result变量(Shadowing)
    let result: String = match result {
        // 正确则调用HttpMethod 已经实现的as_str方法返回httpMethod字符串
        Ok(method) => format!("hello your http method is {}, welcome to request", method.as_str()),
        Err(error_msg) => error_msg.to_string()
    };
    stream.write(result.as_bytes());
    stream.flush().unwrap();
}

fn handle_http_method (content: &str) -> Result<HttpMethod, &str> {
    // 根据请求内容判断HTTP请求method 正确则返回包含HTTPMethod的OK枚举,非HTTPMethod则返回包含错误信息ERR枚举
    if content.starts_with("GET") {
        Result::Ok(HttpMethod::GET)
    } else if content.starts_with("POST") {
        Result::Ok(HttpMethod::POST)
    } else if content.starts_with("HEAD") {
        Result::Ok(HttpMethod::HEAD)
    } else if content.starts_with("PUT") {
        Result::Ok(HttpMethod::PUT)
    } else if content.starts_with("TRACE") {
        Result::Ok(HttpMethod::TRACE)
    } else if content.starts_with("OPTIONS") {
        Result::Ok(HttpMethod::OPTIONS)
    } else if content.starts_with("DELETE") {
        Result::Ok(HttpMethod::DELETE)
    } else {
        Result::Err("invalid http method")
    }
}

enum HttpMethod {
    GET,
    POST,
    HEAD,
    PUT,
    TRACE,
    OPTIONS,
    DELETE
}

impl HttpMethod {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::PUT => "PUT",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::DELETE => "DELETE",
        }
    }
}