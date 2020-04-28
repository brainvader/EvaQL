use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer};
use listenfd::ListenFd;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod schema;
use crate::schema::{create_schema, Schema};

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

    let mut builder = HttpResponse::Ok();
    let response = builder.content_type("application/json").body(user);
    Ok(response)
}

// Each ServiceConfig can have it's own data, routes, and services.
// see https://actix.rs/docs/application/
// taken from https://github.com/lucperkins/rust-graphql-juniper-actix-diesel-postgres/blob/015cf2e116124f8553ee31263ff29ecc8a1bfa3f/src/endpoints.rs
fn graphql_endpoints(config: &mut web::ServiceConfig) {
    let schema = std::sync::Arc::new(create_schema());
    config.data(schema).service(graphql).service(graphiql);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let localhost = std::net::Ipv4Addr::new(127, 0, 0, 1);
    let ip = std::net::IpAddr::V4(localhost);
    let port = 8080;
    let addr = std::net::SocketAddr::new(ip, port);

    let app_factory = move || App::new().configure(graphql_endpoints);

    let mut server = HttpServer::new(app_factory);
    let mut listenfd = ListenFd::from_env();

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(addr)?
    };
    server.run().await
}
