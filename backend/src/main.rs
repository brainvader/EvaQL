use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let localhost = std::net::Ipv4Addr::new(127, 0, 0, 1);
    let ip = std::net::IpAddr::V4(localhost);
    let port = 8080;
    let addr = std::net::SocketAddr::new(ip, port);

    let app_factory = || App::new();

    let server = HttpServer::new(app_factory);
    server.bind(addr)?.run().await
}
