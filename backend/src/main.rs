use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use listenfd::ListenFd;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod schema;
use crate::schema::{create_schema, Schema};
use crate::schema::{Episode, EvaContext, Human};

fn get_status() -> () {
    let mut bayard = std::process::Command::new("bayard");
    let output = bayard
        .arg("status")
        .arg("--server=127.0.0.1:5000")
        .output()
        .unwrap();

    let output_string = String::from_utf8(output.stdout).unwrap();
    log::info!("status: {}", output.status);
    log::info!("outpu: {}", output_string);
    // TODO: Serialize output_literal with serde_json
}

#[get("/graphiql")]
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/graphql")]
async fn graphql(
    state: web::Data<AppState>,
    st: web::Data<std::sync::Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    get_status();

    let my_state = state.get_ref();
    let context = my_state.eva_context.to_owned();
    let user = web::block(move || {
        let res = data.execute(&st, &context);
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

#[derive(Clone)]
struct AppState {
    eva_context: EvaContext,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let localhost = std::net::Ipv4Addr::new(127, 0, 0, 1);
    let ip = std::net::IpAddr::V4(localhost);
    let port = 8080;
    let addr = std::net::SocketAddr::new(ip, port);

    env_logger::init();

    let mut bayard = std::process::Command::new("bayard");
    let child = bayard
        .arg("start")
        .arg("--host=127.0.0.1")
        .arg("--index-port=5000")
        .arg("--schema-file=./db/schema.json")
        .arg("--tokenizer-file=./db/tokenizer.json")
        .arg("1")
        .spawn()?;

    log::info!("child id: {}", child.id());

    let ikari_shinji = Human {
        id: juniper::ID::from("1".to_owned()),
        name: "Ikari Shinji".to_owned(),
        appears_in: vec![Episode::Jo, Episode::Ha, Episode::Q],
    };

    let context = EvaContext {
        human: ikari_shinji,
    };

    let app_state = AppState {
        eva_context: context,
    };

    let app_factory = move || {
        App::new()
            .data(app_state.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
    };

    let mut server = HttpServer::new(app_factory);
    let mut listenfd = ListenFd::from_env();

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(addr)?
    };

    server.run().await
}
