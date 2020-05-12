use juniper::ID;
use juniper::{graphql_value, FieldResult, RootNode};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use serde::{de, Deserialize};

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
    #[serde(deserialize_with = "deserialize_from_array")]
    #[serde(alias = "_id")]
    pub id: ID,
    #[serde(deserialize_with = "deserialize_from_array")]
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

fn deserialize_from_array<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de>,
    D: de::Deserializer<'de>,
{
    struct ActualDataVisitor<T>(std::marker::PhantomData<fn() -> T>);

    impl<'de, T> de::Visitor<'de> for ActualDataVisitor<T>
    where
        T: Deserialize<'de>,
    {
        // Deserialize into
        type Value = T;

        // For error message
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a nonempty sequence of data")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<T, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            // You can use IgnoredAny to skip over the first nth elements.
            let actual_data = seq
                .next_element()?
                .ok_or_else(|| de::Error::custom("no values in seq when looking for maximum"))?;
            Ok(actual_data)
        }
    }
    let visitor = ActualDataVisitor(std::marker::PhantomData);
    deserializer.deserialize_seq(visitor)
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

enum BayardError {
    DocumentNotFound,
}

impl juniper::IntoFieldError for BayardError {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            BayardError::DocumentNotFound => juniper::FieldError::new(
                "Could not found document with given id",
                graphql_value!({
                    "type": "NO_DOCUMENT"
                }),
            ),
        }
    }
}

pub struct QueryRoot;

#[juniper::object(Context = EvaContext)]
impl QueryRoot {
    fn human(id: String) -> Result<Human, BayardError> {
        let (host, port) = get_server_address();
        log::info!("BAYARD_URL: {}:{}", host, port);
        let server_option = format!("--server={}:{}", host, port);
        let mut output = std::process::Command::new("bayard")
            .arg("get")
            .arg(server_option)
            .arg(id)
            .output()
            .unwrap();
        if output.status.success() {
            let output_string = String::from_utf8(output.stdout).unwrap();
            log::info!("output_string: {:?}", output_string);
            let output_json: Human = serde_json::from_str(&output_string).unwrap();
            log::info!("out_json: {:?}", output_json);
            let Human {
                id,
                name,
                appears_in,
            } = &output_json;
            return Ok(Human {
                id: id.to_owned(),
                name: name.to_string(),
                appears_in: appears_in.to_vec(),
            });
        }
        Err(BayardError::DocumentNotFound)
        // Ok(Human {
        //     id: ID::from("1".to_owned()),
        //     name: "Shinji Ikari".to_owned(),
        //     appears_in: vec![Episode::Jo, Episode::Ha, Episode::Q],
        // })
    }
}

#[derive(GraphQLInputObject)]
struct NewHuman {
    id: ID,
    name: String,
    appears_in: Vec<Episode>,
}

pub struct MutationRoot;

#[juniper::object(Context = EvaContext)]
impl MutationRoot {
    fn createHuman(new_human: NewHuman) -> FieldResult<Human> {
        let human = Human {
            id: ID::from(new_human.id.to_owned()),
            name: new_human.name.to_owned(),
            appears_in: new_human.appears_in.clone(),
        };
        log::info!("{:?}", human);
        Ok(human)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
