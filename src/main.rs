pub mod http_srv;

fn main() {
    let mut http_server = http_srv::HttpServer{};
    http_server.run("127.0.0.1".to_string(), 8080)
}