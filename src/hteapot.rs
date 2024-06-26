// Written by Alberto Ruiz 2024-03-08
// This is the HTTP server module, it will handle the requests and responses
// Also provide utilities to parse the requests and build the responses
//
// Info: This will be turn into a library

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use rayon::ThreadPoolBuilder;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum HttpMethod {
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

#[derive(Clone, Copy)]
pub enum HttpStatus {
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


impl HttpStatus {
    fn to_string(&self) -> &str {
        match self {
            HttpStatus::OK => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::Accepted => "Accepted",
            HttpStatus::NoContent => "No Content",
            HttpStatus::MovedPermanently => "Moved Permanently",
            HttpStatus::MovedTemporarily => "Moved Temporarily",
            HttpStatus::NotModified => "Not Modified",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::IAmATeapot => "I'm a teapot",
            HttpStatus::InternalServerError => "Internal Server Error",
            HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::BadGateway => "Bad Gateway",
            HttpStatus::ServiceUnavailable => "Service Unavailable",
        }
    }

}


pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub args: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
}


pub struct HteaPot {
    port: u16,
    address: String,
    // this will store a map from path to their actions
    // path_table: HashMap<HttpMethod, HashMap<String, HashMap<HttpMethod, fn(HttpRequest) -> String>>>,
}

impl HteaPot {

    // Constructor
    pub fn new(address: &str, port: u16) -> Self {
        HteaPot {
            port: port,
            address: address.to_string(),
            // path_table: HashMap::new(),
        }
    }

    // Start the server
    pub fn listen(&self, action: impl Fn(HttpRequest) -> String + Send + Sync + 'static ){
        let addr = format!("{}:{}", self.address, self.port);
        let listener = TcpListener::bind(addr);
        let listener = match listener {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };
        let action_clone = Arc::new(action);
        for stream in listener.incoming() {
            match stream {
                 Ok(stream) => {
                    let action_clone = action_clone.clone();
                    thread::spawn(move || {
                                HteaPot::handle_client(stream, |req| {
                                    action_clone(req)
                                });
                    });
   
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }


    // Create a response
    pub fn response_maker(status: HttpStatus, content: &str) -> String {
        let status_text = status.to_string();
        let content_length = format!("Content-Length: {}", content.len());
        let response = format!("HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",status as u16, status_text,content_length ,content);
        response
    }

    // Parse the request
    pub fn request_parser(request: &str) -> HttpRequest {
        let mut lines = request.lines();
        let first_line = lines.next().unwrap();
        let mut words = first_line.split_whitespace();
        let method = words.next().unwrap();
        let mut path = words.next().unwrap().to_string();
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
        let body = body.trim().trim_end();
        //remove all traling zero bytes
        let body = body.trim_matches(char::from(0));
        let mut args: HashMap<String, String> = HashMap::new();
        //remove http or https from the path
        if path.starts_with("http://") {
            path = path.trim_start_matches("http://").to_string();
        } else if path.starts_with("https://") {
            path = path.trim_start_matches("https://").to_string();
        }
        //remove the host name if present
        if !path.starts_with("/") {
            //remove all the characters until the first /
            let mut parts = path.split("/");
            parts.next();
            path = parts.collect::<Vec<&str>>().join("/");
            //add / to beggining
            path = format!("/{}", path);
        }

        if path.contains('?') {
            let _path = path.clone();
            let mut parts = _path.split('?');
            path = parts.next().unwrap().to_string();
            let query = parts.next().unwrap();
            let query_parts: Vec<&str> = query.split('&').collect();
            for part in query_parts {
                let mut parts = part.split('=');
                let key = parts.next().unwrap().to_string();
                let value = parts.next().unwrap_or("").to_string().replace("%22", "\"");
                args.insert(key, value);
            }
        }
        let body = body.trim_end().to_string();
        let expected_size = headers.get("Content-Length").unwrap_or(&"-1".to_string()).parse::<i32>().unwrap();
        let body_size = body.len();
        println!("expected: {}\nrecived: {}",expected_size,body_size);
        HttpRequest {
            method: HttpMethod::from_str(method),
            path: path.to_string(),
            args: args,
            headers: headers,
            body: body.trim_end().to_string(),
        }
    }

    // Handle the client when a request is received
    fn handle_client(mut stream: TcpStream , action: impl Fn(HttpRequest) -> String ) {
        let mut request_buffer: String = String::new();
        loop {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap_or_default();
            if buffer[0] == 0 {break};
            let partial_request_buffer = String::from_utf8_lossy(&buffer).to_string();
            request_buffer.push_str(&partial_request_buffer);
            if *buffer.last().unwrap() == 0 {break;}
        }
        
        let request = Self::request_parser(&request_buffer);
        println!("Received request: \n{} {}\n\n", request.method.to_str(), request.path);
        //let response = Self::response_maker(HttpStatus::IAmATeapot, "Hello, World!");
        let response = action(request);
        let r = stream.write(response.as_bytes()); 
        if r.is_err() {
            eprintln!("Error: {}", r.err().unwrap());
        }
        let r = stream.flush();
        if r.is_err() {
            eprintln!("Error: {}", r.err().unwrap());
        }
    }
}


#[cfg(test)]

#[test]
fn test_http_parser() {
    let request = "GET / HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/7.68.0\r\nAccept: */*\r\n\r\n";
    let parsed_request = HteaPot::request_parser(request);
    assert_eq!(parsed_request.method, HttpMethod::GET);
    assert_eq!(parsed_request.path, "/");
    assert_eq!(parsed_request.args.len(), 0);
    assert_eq!(parsed_request.headers.len(), 3);
    assert_eq!(parsed_request.body, "");
}

#[test]
fn test_http_response_maker() {
    let response = HteaPot::response_maker(HttpStatus::IAmATeapot, "Hello, World!");
    let expected_response = "HTTP/1.1 418 I'm a teapot\r\nContent-Length: 13\r\n\r\nHello, World!";
    assert_eq!(response, expected_response);
}

