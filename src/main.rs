use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
mod engine;
mod memodb;
use engine::Engine;
enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
}


impl HttpMethod {
    fn from_str(method: &str) -> HttpMethod {
        match method {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "PATCH" => HttpMethod::PATCH,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            "TRACE" => HttpMethod::TRACE,
            "CONNECT" => HttpMethod::CONNECT,
            _ => panic!("Invalid HTTP method"),
        }
    }
    fn to_str(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::CONNECT => "CONNECT",
        }
    }
}

enum HttpStatus {
    OK = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    MovedPermanently = 301,
    MovedTemporarily = 302,
    NotModified = 304,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    IAmATeapot = 418,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}

pub struct HttpRequest {
    method: HttpMethod,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}



fn response_maker(status: HttpStatus, content: &str) -> String {
    let status_text = status as u16;
    let response = format!("HTTP/1.1 {} OK\r\n\r\n{}",status_text ,content);
    response
}

fn request_parser(request: &str) -> HttpRequest {
    let mut lines = request.lines();
    let first_line = lines.next().unwrap();
    let mut words = first_line.split_whitespace();
    let method = words.next().unwrap();
    let path = words.next().unwrap();
    let mut headers: HashMap<String, String> = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(": ");
        let key = parts.next().unwrap().to_string();
        let value = parts.next().unwrap();
        headers.insert(key, value.to_string());
    }
    let remaining_lines: Vec<&str>  = lines.collect();
    let body = remaining_lines.join("");
    HttpRequest {
        method: HttpMethod::from_str(method),
        path: path.to_string(),
        headers: headers,
        body: body.trim_end().to_string(),
    }
}

fn handle_client(mut stream: TcpStream, mut engine: Rc<RefCell<Engine>>) {
    let mut buffer = [0; 1024];
    let mut engine = engine.borrow_mut();
    stream.read(&mut buffer).unwrap();
    let request_buffer = String::from_utf8_lossy(&buffer);
    let request = request_parser(&request_buffer);
    //println!("Received request: \n{}\n\n", request_buffer);
    println!("Parsed method: {:?}\n\n", request.method.to_str());
    println!("Parsed path: {:?}\n\n", request.path);
    let response = engine.process(request);
    //let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    print!("Starting server...");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut engine: Rc<RefCell<Engine>> = Rc::new(RefCell::new(Engine::new()));
    engine.borrow_mut().init_mock_data();
    println!("started!");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream, engine.clone());
    }
}
