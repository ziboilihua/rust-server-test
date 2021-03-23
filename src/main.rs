use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn main() {
    // 在本地7878端口创建TCP连接
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 读取异常响应文件html
    let res_html = fs::read_to_string("error.html").unwrap();
    // 生成异常响应信息字符串
    let err_resp = format!(
        "HTTP/1.1 404 NOTFOUND\r\nContent-Length: {}\r\n\r\n{}",
        res_html.len(),
        res_html
    );
    // 读取正常响应文件html res_html 变量隐藏
    let res_html = fs::read_to_string("hello.html").unwrap();
    // 生成正常返回信息字符串
    let success_resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        res_html.len(),
        res_html
    );
    // 监听TCP连接
    for stream in listener.incoming() {
        // 获取tcpStream (如果失败直接panic)
        let mut stream = stream.unwrap();
        // 打印请求信息
        let req = print_request(&stream);
        // 通过请求信息判断http请求
        let result = handle_http_method(req.as_str());
        // 生成响应信息
        let resp = match result {
            Ok(m) => {
                print!("request method: {}", m.as_str());
                success_resp.as_str()
            },
            Err(_) => err_resp.as_str()
        };
        // 将响应信息写入到流中
        stream.write(resp.as_bytes());
        // 刷新流
        stream.flush().unwrap();
    }
}


fn print_request(mut stream: &TcpStream) -> String {
    // 在栈上声明一个 buffer 来存放读取到的数据，创建缓冲区的大小为1024字节
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // 将缓冲区的字节转换为字符串
    let content = String::from_utf8_lossy(&buffer);
    // 打印请求内容
    print!("{}", content);
    // 返回请求信息
    content.to_string()
}


fn handle_http_method (content: &str) -> Result<HttpMethod, &str> {
    // 根据请求内容判断HTTP请求method 正确则返回包含HTTPMethod的OK枚举,非GET POST则返回包含错误信息ERR枚举
    if content.starts_with("GET") {
        Result::Ok(HttpMethod::GET)
    } else if content.starts_with("POST") {
        Result::Ok(HttpMethod::POST)
    } else {
        Result::Err("http method not support")
    }
}

enum HttpMethod {
    GET,
    POST
}

impl HttpMethod {
    // 实现as_str方法便于打印
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST"
        }
    }
}