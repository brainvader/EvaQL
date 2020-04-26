use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer};

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

#[get("/graphiql")]
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/graphql")]
async fn graphql(
    st: web::Data<std::sync::Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;

    let builder = HttpResponse::Ok();
    let response = builder.content_type("application/json").body(user);
    Ok(response)
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
