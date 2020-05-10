use juniper::ID;
use juniper::{FieldResult, RootNode};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use serde::Deserialize;

#[derive(GraphQLEnum, Clone, Deserialize, Debug)]
pub enum Episode {
    #[serde(alias = "/episode/Jo")]
    Jo,
    #[serde(alias = "/episode/Ha")]
    Ha,
    #[serde(alias = "/episode/Q")]
    Q,
}

#[derive(GraphQLObject, Clone, Deserialize, Debug)]
#[graphql(description = "A human being in the Rebuild of Evangelion")]
pub struct Human {
    #[serde(alias = "_id")]
    pub id: ID,
    pub name: String,
    pub appears_in: Vec<Episode>,
}

#[derive(GraphQLObject)]
#[graphql(description = "An angl creature in the Rebuild of Evangelion")]
struct Angel {
    id: ID,
    name: String,
    appears_in: Vec<Episode>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A cyborg in the Rebuild of Evangelion")]
struct Evangelion {
    id: ID,
    name: String,
    appears_in: Vec<Episode>,
}

#[derive(Clone)]
pub struct EvaContext {
    pub human: Human,
}

impl juniper::Context for EvaContext {}

fn load_env_file() {
    let env_file = dotenv::dotenv().unwrap();
    match env_file.to_str() {
        Some(path_name) => log::info!("path name: {}", path_name),
        None => log::info!("No .env file"),
    }
}

fn get_server_address() -> (std::string::String, std::string::String) {
    load_env_file();
    let host = std::env::var("BAYARD_HOST").unwrap();
    let port = std::env::var("BAYARD_PORT").unwrap();
    (host, port)
}

// JSON Processor function
fn jq(output: &str) -> serde_json::Value {
    serde_json::from_str(output).unwrap()
}

pub struct QueryRoot;

#[juniper::object(Context = EvaContext)]
impl QueryRoot {
    fn human(context: &EvaContext) -> FieldResult<Human> {
        let (host, port) = get_server_address();
        log::info!("BAYARD_URL: {}:{}", host, port);
        let server_option = format!("--server={}:{}", host, port);
        let mut output = std::process::Command::new("bayard")
            .arg("get")
            .arg(server_option)
            .arg("1")
            .output()?;
        let output_string = String::from_utf8(output.stdout).unwrap();
        log::info!("output_string: {:?}", output_string);
        let output_json: Human = serde_json::from_str(&output_string).unwrap();
        log::info!("out_json: {:?}", output_json);
        let Human {
            id,
            name,
            appears_in,
        } = &context.human;
        Ok(Human {
            id: id.to_owned(),
            name: name.to_string(),
            appears_in: appears_in.to_vec(),
        })
        // Ok(Human {
        //     id: ID::from("1".to_owned()),
        //     name: "Shinji Ikari".to_owned(),
        //     appears_in: vec![Episode::Jo, Episode::Ha, Episode::Q],
        // })
    }
}

pub struct MutationRoot;

#[juniper::object(Context = EvaContext)]
impl MutationRoot {
    fn createHuman(id: ID) -> FieldResult<Human> {
        Ok(Human {
            id: ID::from("1".to_owned()),
            name: "Shinji Ikari".to_owned(),
            appears_in: vec![Episode::Jo, Episode::Ha, Episode::Q],
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
