// Written by Alberto Ruiz 2024-03-08
// This is the HTTP server module, it will handle the requests and responses
// Also provide utilities to parse the requests and build the responses
//
// Info: This will be turn into a library

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

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
    pub fn listen(&self, action: impl Fn(HttpRequest) -> String ){
        let addr = format!("{}:{}", self.address, self.port);
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            match stream {
                 Ok(stream) => {
                //     thread::spawn(move || {
                //         HteaPot::handle_client(stream);
                //     });
                    self.handle_client(stream, &action)
   
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }


    // Create a response
    pub fn response_maker(status: HttpStatus, content: &str) -> String {
        let status_text = status as u16;
        let response = format!("HTTP/1.1 {} OK\r\n\r\n{}",status_text ,content);
        response
    }

    // Parse the request
    pub fn request_parser(request: &str) -> HttpRequest {
        let mut lines = request.lines();
        let first_line = lines.next().unwrap();
        let mut words = first_line.split_whitespace();
        let method = words.next().unwrap();
        let mut path = words.next().unwrap();
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
        let mut args: HashMap<String, String> = HashMap::new();
        if path.contains('?') {
            let mut parts = path.split('?');
            path = parts.next().unwrap();
            let query = parts.next().unwrap();
            let query_parts: Vec<&str> = query.split('&').collect();
            for part in query_parts {
                let mut parts = part.split('=');
                let key = parts.next().unwrap().to_string();
                let value = parts.next().unwrap_or("").to_string().replace("%22", "\"");
                args.insert(key, value);
            }
        }

        HttpRequest {
            method: HttpMethod::from_str(method),
            path: path.to_string(),
            args: args,
            headers: headers,
            body: body.trim_end().to_string(),
        }
    }

    // Handle the client when a request is received
    fn handle_client(&self, mut stream: TcpStream , action: impl Fn(HttpRequest) -> String ) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap(); //TODO: handle the error
        let request_buffer = String::from_utf8_lossy(&buffer);
        let request = Self::request_parser(&request_buffer);
        println!("Received request: \n{} {}\n\n", request.method.to_str(), request.path);
        //let response = Self::response_maker(HttpStatus::IAmATeapot, "Hello, World!");
        let response = action(request);
        stream.write(response.as_bytes()).unwrap(); //TODO: handle the error
        stream.flush().unwrap(); //TODO: handle the error
    }
}
