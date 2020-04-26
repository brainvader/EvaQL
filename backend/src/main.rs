use actix_web::{get, App, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;

#[get("/graphiql")]
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let localhost = std::net::Ipv4Addr::new(127, 0, 0, 1);
    let ip = std::net::IpAddr::V4(localhost);
    let port = 8080;
    let addr = std::net::SocketAddr::new(ip, port);

    let app_factory = || App::new().service(graphiql);

    let server = HttpServer::new(app_factory);
    server.bind(addr)?.run().await
}
