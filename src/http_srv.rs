use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs::{read_dir, read_to_string}
};

pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub user_agent: String
}

pub struct HttpResult {
    pub code: i32,
    pub content: String,
    pub content_type: String
}

pub struct HttpServer { }

impl HttpServer {
    pub fn run(&mut self, addr: String, port: i32) {
        let listener = TcpListener::bind(format!("{addr}:{port}")).unwrap();
        println!("Server started on {}:{}", addr, port);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        println!("New connection from {}", stream.peer_addr().unwrap().to_string());

        let buf_reader = BufReader::new(&mut stream);
        let data: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        let http_request_opt = self.parse_request(data);
    
        if http_request_opt.is_some() {
            let http_request: HttpRequest = http_request_opt.unwrap();
            println!("====[New request]====");
            println!("{} {}", http_request.method, http_request.url);
            println!("User agent: {}", http_request.user_agent);
            println!("=====================");
        
            let mut files_html = "".to_owned();
            let files = read_dir("./assets/").unwrap();
            let mut founded_file_path = "".to_string();
            for file in files {
                let path = file.unwrap().path().display().to_string().replace("./assets", "");
                files_html.push_str(format!("<br><a href='{path}'>{path}</a>").as_str());
        
                if http_request.url == path {
                    founded_file_path = path;
                }
            }
        
            if founded_file_path == "" {
                let result = self.build_result(HttpResult { code: 200, content: files_html, content_type: "text/html; charset=utf-8".to_string() });
                let _ = stream.write_all(result.as_bytes());
                return;
            } else {
                let content = read_to_string(format!("./assets{founded_file_path}")).unwrap();
                let result = self.build_result(HttpResult { code: 200, content: content, content_type: "text/html; charset=utf-8".to_string() });
                let _ = stream.write_all(result.as_bytes());
                return;
            }
        }
    
        
    }
    
    fn parse_request(&mut self, data: Vec<String>) -> Option<HttpRequest> {
        if data.len() < 1 {
            println!("Wrong request");
            return None;
        }
    
        let first_line: Vec<&str> = data[0].split(" ").collect();
        let method = first_line[0];
        let url = first_line[1];
        let mut user_agent = "";
        for prop in &data {
            if prop.contains(":") {
                let items: Vec<&str> = prop.split(":").collect();
                let key = items[0];
                let value = items[1];
    
                if key == "User-Agent" {
                    user_agent = value;
                }
            }
        }
    
        return Some(HttpRequest{ method: method.to_string(), url: url.to_string(), user_agent: user_agent.to_string() });
    }
    
    fn build_result(&mut self, result: HttpResult) -> String {
        let code = result.code;
        let content = result.content;
        let content_length = content.len();
        let content_type = result.content_type;
        let header = format!("Content-Length: {content_length}\r\nContent-Type: {content_type}\r\n");
    
        let response = format!("HTTP/1.1 {code} OK\r\n{header}\r\n{content}");
        return response;
    }
}